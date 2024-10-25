use std::str::FromStr;

use aoc_traits::prelude::*;

use crate::Year2022Solver;

use super::camp::Camp;

impl Solver<Year2022, Day04, Part02> for Year2022Solver {
    fn solve<T: AsRef<str>>(&self, input: T) -> AocResult<Flag> {
        let camp = Camp::from_str(input.as_ref())?;
        let count = camp
            .as_ref()
            .iter()
            .filter(|pair| pair.is_self_overlapping())
            .count();

        Ok(Flag::new(count as i64))
    }
}
