#![allow(unused)]

use attribute_derive::{Attribute, AttributeIdent};
use proc_macro::{self, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::{collections::HashMap, fmt::Debug};
use syn::{DeriveInput, Field, Ident, Result};

#[derive(Attribute, Debug)]
#[attribute(ident = ligma_schema)]
// #[attribute(invalid_field = "ok")]
struct LigmaSchema {
    service: String,
    table: String,
    entity: String,

    hash: String,
    range: String,

    gsi1: Option<String>,
    gsi1_hash: Option<String>,
    gsi1_range: Option<String>,

    gsi2: Option<String>,
    gsi2_hash: Option<String>,
    gsi2_range: Option<String>,
}

#[derive(Attribute, Debug)]
#[attribute(ident = ligma_attribute)]
struct LigmaAttribute {
    index: String,
    key: String,
    #[attribute(default = 0)]
    position: usize,
}

// todo: cant use empty struct???
#[derive(Attribute, Debug)]
#[attribute(ident = ligma_ignore)]
struct LigmaIgnore {
    #[attribute(optional = false, default = true)]
    ignore: bool,
}

// fn all_attrs<T: Attribute + AttributeIdent + Debug>(input: TokenStream) -> Result<TokenStream> {
//     let DeriveInput { attrs, data, .. } = syn::parse(input)?;
//     let a = T::from_attributes(&attrs)?;
//     println!("{:#?}", a);
//     Ok(TokenStream::new())
// }

// enum Index {
//     Primary,
//     Gsi1,
//     Gsi2,
// }

struct IndexKeys {
    index: Ident,
    hash: Key,
    range: Key,
}

// enum IndexKey {
//     Hash(Key),
//     Range(Key),
// }

#[derive(Default)]
struct Key {
    field: String,
    composite: Vec<Composite>,
}

struct Composite {
    position: usize,
    syn_field: Field,
}

macro_rules! insert_index {
    ($map: ident, $name: expr, $hash: expr, $range: expr, $index: expr) => {
        $map.insert(
            $name,
            IndexKeys {
                // index: Index::$index,
                index: format_ident!($index),
                hash: Key {
                    field: $hash,
                    ..Default::default()
                },
                range: Key {
                    field: $range,
                    ..Default::default()
                },
            },
        );
    };
}

macro_rules! insert_gsi {
    ($map: ident, $name: expr, $hash: expr, $range: expr, $index: expr) => {
        if let Some(g) = $name {
            insert_index!($map, g, $hash.unwrap(), $range.unwrap(), $index);
        }
    };
}

#[proc_macro_derive(LigmaEntity, attributes(ligma_schema, ligma_attribute, ligma_ignore))]
pub fn derive(input: TokenStream) -> TokenStream {
    // let a = all_attrs::<LigmaSchema>(input).unwrap_or_else(|e| e.to_compile_error().into());
    // let a = all_attrs::<LigmaSchema>(input).unwrap();

    let DeriveInput {
        attrs,
        data,
        ident,
        generics,
        ..
    } = syn::parse(input).unwrap();

    let s = LigmaSchema::from_attributes(&attrs).unwrap();

    let mut m = HashMap::new();
    insert_index!(m, "primary".to_string(), s.hash, s.range, "Primary");
    insert_gsi!(m, s.gsi1, s.gsi1_hash, s.gsi1_range, "Gsi1");
    insert_gsi!(m, s.gsi2, s.gsi2_hash, s.gsi2_range, "Gsi2");

    let struct_data = match data {
        syn::Data::Struct(s) => s,
        _ => panic!(), // not a struct
    };

    for field in struct_data.fields.iter() {
        if field.attrs.len() > 0 {
            if let Ok(attribute) = LigmaIgnore::from_attributes(&field.attrs) {
                if attribute.ignore {
                    continue;
                }
            }

            if let Ok(attribute) = LigmaAttribute::from_attributes(&field.attrs) {
                let composite = Composite {
                    position: attribute.position,
                    syn_field: field.clone(),
                };

                if let Some(index) = m.get_mut(&attribute.index) {
                    match attribute.key.as_str() {
                        "hash" => index.hash.composite.push(composite),
                        "range" => index.range.composite.push(composite),
                        _ => panic!(), // key must be hash or range
                    }
                } else {
                    panic!(); // unknown index
                }
            }
        }

        let type_name = match &field.ty {
            syn::Type::Path(p) => p.to_token_stream().to_string(),
            _ => panic!(), // could not parse as type path?
        };

        match type_name.as_str() {
            "String" | "f64" | "bool" => {}
            _ => panic!(), // unsupported type
        }
    }

    let mut index_match_arms = quote! {};
    for (k, v) in m.iter() {
        let index = v.index.clone();
        let service = s.service.clone();
        let entity = s.entity.clone();

        let mut hash_composite = quote! {};
        for (i, _) in v.hash.composite.iter().enumerate() {
            let composite = v.hash.composite.iter().find(|c| c.position == i).unwrap();
            let ident = composite.syn_field.ident.clone().unwrap();
            let ident_string = ident.to_string();

            hash_composite = quote! {
                #hash_composite
                something.push_str(&format!("#{}_{}", #ident_string, self.#ident));
            };
        }

        let mut range_composite = quote! {};
        for (i, _) in v.range.composite.iter().enumerate() {
            let composite = v.range.composite.iter().find(|c| c.position == i).unwrap();
            let ident = composite.syn_field.ident.clone().unwrap();
            let ident_string = ident.to_string();

            range_composite = quote! {
                #range_composite
                something.push_str(&format!("#{}_{}", #ident_string, self.#ident));
            };
        }

        index_match_arms = quote! {
            #index_match_arms
            Index::#index => {
                let mut something = String::new();
                match key {
                    Key::Hash => {
                        something.push_str(&format!("${}#{}", #service, #entity));
                        #hash_composite
                    }
                    Key::Range => {
                        something.push_str(&format!("${}", #entity));
                        #range_composite
                    }
                }
                println!("{}", something);
                println!("{}", something);
                println!("{}", something);
                println!("{}", something);
                println!("{}", something);
            }
        };
    }

    let imports = quote! {
        use ligmacro::{Index, Key};
    };

    let (ig, tg, wc) = generics.split_for_impl();
    let impl_ligma = quote! {
        impl #ig LigmaEntity for #ident #tg #wc {
            fn index_key(&self, index: Index, key: Key) {
                match index {
                    #index_match_arms
                    _ => {
                        // todo: unknown index for entity
                        println!("unknown index for entity");
                    }
                }
            }
        }
    };

    let o = quote! {
        #imports
        #impl_ligma
    };

    o.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
