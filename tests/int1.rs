use ligmacro::LigmaEntity;
use std::collections::HashMap;

struct Bar {
    pub deez: String,
}

#[derive(LigmaEntity)]
#[ligma_schema(table = "foo_table", service = "foo_service", entity = "foo_entity")]
#[ligma_schema(hash = "pk", range = "sk")]
#[ligma_schema(gsi1 = "gsi1", gsi1_hash = "gsi1pk", gsi1_range = "gsi1sk")]
struct Foo {
    #[ligma_attribute(index = "primary", key = "hash")]
    foo_string1: String,
    #[ligma_attribute(index = "primary", key = "range")]
    foo_string2: String,
    #[ligma_attribute(index = "primary", key = "range", position = 1)]
    foo_string3: String,
    #[ligma_attribute(index = "gsi1", key = "hash")]
    foo_string4: String,
    foo_string5: String,
    foo_string6: String,
    foo_bool: bool,
    #[ligma_attribute(index = "gsi1", key = "range")]
    foo_num1: f64,
    #[ligma_ignore(ignore)]
    bar: Bar,
}

#[test]
fn test1() {
    let a = Foo {
        foo_string1: "asdf".to_string(),
        foo_string2: "fdsa".to_string(),
        foo_string3: "aaaa".to_string(),
        foo_string4: "bbbb".to_string(),
        foo_string5: "cccc".to_string(),
        foo_string6: "dddd".to_string(),
        foo_num1: 69.0,
        bar: Bar {
            deez: "ddez".to_string(),
        },
        foo_bool: true,
    };

    let x: HashMap<String, AttributeValue> = a.into();
    println!("{:#?}", x);

    // let b = a.index_key(Index::Gsi1, Key::Range);
    // let c = a.index_keys(Index::Primary);

    // println!("{:#?}", b);
    // println!("{:#?}", c);

    // let e = FooIndex::Gsi1;
    // let aaa = FOO_GSI1;
    // let bbb = Foo_
}
