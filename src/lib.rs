pub use deez_derive::Deez;

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Index {
    Primary,
    Gsi1,
    Gsi2,
    Gsi3,
    Gsi4,
    Gsi5,
    Gsi6,
    Gsi7,
    Gsi8,
    Gsi9,
    Gsi10,
    Gsi11,
    Gsi12,
    Gsi13,
    Gsi14,
    Gsi15,
    Gsi16,
    Gsi17,
    Gsi18,
    Gsi19,
    Gsi20,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Key {
    Hash,
    Range,
}

#[derive(Debug)]
pub struct IndexKey {
    pub field: String,
    pub composite: String,
}

#[derive(Debug)]
pub struct IndexKeys {
    pub hash: IndexKey,
    pub range: IndexKey,
}

// todo: better tests lmao
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
