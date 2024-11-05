use std::{iter::Sum, ops::Add};

use nutype::nutype;

#[nutype(derive(Debug, Clone, Copy, PartialEq, Eq, Hash))]
pub struct Item(char);

impl Item {
    pub fn priority(&self) -> Priority {
        let ascii_code = self.into_inner() as i64;
        // Map ascii codes to instruction ones
        let priority = match self.into_inner() {
            // Ascii code of 'a' is 97, and we add 1 to align with instructions
            'a'..='z' => ascii_code - 97 + 1,
            // Ascii code of 'A' is 65, and we add 27 to align with instructions
            'A'..='Z' => ascii_code - 65 + 27,
            _ => unreachable!(),
        };

        Priority::new(priority)
    }
}

#[nutype(derive(Debug, Default, Clone, Copy, PartialEq, Eq), default = 0)]
pub struct Priority(i64);

impl Add for Priority {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.into_inner() + rhs.into_inner())
    }
}

impl Sum for Priority {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|total, current| total + current)
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority_conversion() {
        let item = Item::new('a');
        let priority = item.priority();

        assert_eq!(priority.into_inner(), 1);
    }
}
