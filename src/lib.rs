pub use deez_derive::Deez;

#[derive(Debug)]
pub struct IndexKeys {
    pub hash: IndexKey,
    pub range: IndexKey,
}

#[derive(Debug)]
pub struct IndexKey {
    pub field: String,
    pub composite: String,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Key {
    Hash,
    Range,
}

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

impl std::fmt::Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Index::Primary => write!(f, "Primary"),
            Index::Gsi1 => write!(f, "Gsi1"),
            Index::Gsi2 => write!(f, "Gsi2"),
            Index::Gsi3 => write!(f, "Gsi3"),
            Index::Gsi4 => write!(f, "Gsi4"),
            Index::Gsi5 => write!(f, "Gsi5"),
            Index::Gsi6 => write!(f, "Gsi6"),
            Index::Gsi7 => write!(f, "Gsi7"),
            Index::Gsi8 => write!(f, "Gsi8"),
            Index::Gsi9 => write!(f, "Gsi9"),
            Index::Gsi10 => write!(f, "Gsi10"),
            Index::Gsi11 => write!(f, "Gsi11"),
            Index::Gsi12 => write!(f, "Gsi12"),
            Index::Gsi13 => write!(f, "Gsi13"),
            Index::Gsi14 => write!(f, "Gsi14"),
            Index::Gsi15 => write!(f, "Gsi15"),
            Index::Gsi16 => write!(f, "Gsi16"),
            Index::Gsi17 => write!(f, "Gsi17"),
            Index::Gsi18 => write!(f, "Gsi18"),
            Index::Gsi19 => write!(f, "Gsi19"),
            Index::Gsi20 => write!(f, "Gsi20"),
        }
    }
}
