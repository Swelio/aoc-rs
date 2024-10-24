use std::{iter::Sum, ops::Add};

use nutype::nutype;

#[nutype(
    derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord),
    default = 0
)]
pub struct Score(i64);

impl Add for Score {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.into_inner() + rhs.into_inner())
    }
}

impl Sum for Score {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(Score::add).unwrap_or_default()
    }
}
