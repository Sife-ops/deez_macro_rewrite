pub use ligmacro_derive::LigmaEntity;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Index {
    Primary,
    Gsi1,
    Gsi2,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Key {
    Hash,
    Range,
}

pub trait LigmaEntity {
    fn index_key(&self, index: Index, key: Key);
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
