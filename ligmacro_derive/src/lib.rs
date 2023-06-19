#![allow(unused)]

use attribute_derive::{Attribute, AttributeIdent};
use proc_macro::{self, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::{collections::HashMap, fmt::Debug};
use syn::{DeriveInput, Field, Ident, Result};

////////////////////////////////////////////////////////////////////////////////
/// attribute helpers
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
////////////////////////////////////////////////////////////////////////////////

struct IndexKeys {
    index: Ident,
    hash: IndexKey,
    range: IndexKey,
}

#[derive(Default)]
struct IndexKey {
    field: String,
    composite: Vec<Composite>,
}

struct Composite {
    position: usize,
    syn_field: Field,
}

////////////////////////////////////////////////////////////////////////////////
/// helper macros
macro_rules! insert_index {
    ($map: ident, $name: expr, $hash: expr, $range: expr, $index: expr) => {
        $map.insert(
            $name,
            IndexKeys {
                // index: Index::$index,
                index: format_ident!($index),
                hash: IndexKey {
                    field: $hash,
                    ..Default::default()
                },
                range: IndexKey {
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

macro_rules! compose_key {
    ($index_key: expr) => {{
        let mut composed = quote! {};
        for (i, _) in $index_key.composite.iter().enumerate() {
            let composite = $index_key
                .composite
                .iter()
                .find(|c| c.position == i)
                .unwrap();
            let ident = composite.syn_field.ident.clone().unwrap();
            let ident_string = ident.to_string();

            composed = quote! {
                #composed
                composed.push_str(&format!("#{}_{}", #ident_string, self.#ident));
            };
        }
        composed
    }};
}
////////////////////////////////////////////////////////////////////////////////

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

    let mut index_key_match = quote! {};
    let mut index_keys_match = quote! {};
    for (k, v) in m.iter() {
        let hash_composite = compose_key!(v.hash);
        let range_composite = compose_key!(v.range);

        let index = v.index.clone();
        let service = s.service.clone();
        let entity = s.entity.clone();
        let hash_field = v.hash.field.clone();
        let range_field = v.range.field.clone();

        index_key_match = quote! {
            #index_key_match
            Index::#index => {
                let mut composed = String::new();
                match key {
                    Key::Hash => {
                        composed.push_str(&format!("${}#{}", #service, #entity));
                        #hash_composite
                        return IndexKey {
                            field: #hash_field.to_string(),
                            composite: composed,
                        }
                    }
                    Key::Range => {
                        composed.push_str(&format!("${}", #entity));
                        #range_composite
                        return IndexKey {
                            field: #range_field.to_string(),
                            composite: composed,
                        }
                    }
                }
            }
        };

        index_keys_match = quote! {
            #index_keys_match
            Index::#index => {
                return IndexKeys {
                    hash: self.index_key(Index::#index, Key::Hash),
                    range: self.index_key(Index::#index, Key::Range),
                }
            }
        }
    }

    let uses = quote! {
        use ligmacro::{Index, Key, IndexKey, IndexKeys};
    };

    let (ig, tg, wc) = generics.split_for_impl();
    let impl_ligma = quote! {
        impl #ig LigmaEntity for #ident #tg #wc {
            fn index_key(&self, index: Index, key: Key) -> IndexKey {
                match index {
                    #index_key_match
                    _ => panic!() // unknown index for entity
                }
            }
            fn index_keys(&self, index: Index) -> IndexKeys {
                match index {
                    #index_keys_match
                    _ => panic!() // unknown index for entity
                }
            }
        }
    };

    let o = quote! {
        #uses
        #impl_ligma
    };

    o.into()
}

// dead code!
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

// fn all_attrs<T: Attribute + AttributeIdent + Debug>(input: TokenStream) -> Result<TokenStream> {
//     let DeriveInput { attrs, data, .. } = syn::parse(input)?;
//     let a = T::from_attributes(&attrs)?;
//     println!("{:#?}", a);
//     Ok(TokenStream::new())
// }
