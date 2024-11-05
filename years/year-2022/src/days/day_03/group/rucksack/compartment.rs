use std::collections::HashSet;

use nutype::nutype;

use super::item::Item;

#[nutype(derive(Debug, Clone, AsRef, PartialEq, Eq))]
pub struct Compartment(HashSet<Item>);

impl Compartment {
    pub fn intersection(&self, other: &Self) -> Self {
        Self::new(
            self.as_ref()
                .intersection(other.as_ref())
                .copied()
                .collect(),
        )
    }
}
