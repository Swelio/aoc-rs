use std::{cmp::Reverse, str::FromStr};

use aoc_traits::prelude::*;

use crate::solver::Year2022Solver;

use super::input::{Calories, Input};

impl Solver<Year2022, Day01, Part02> for Year2022Solver {
    fn solve<T: AsRef<str>>(&self, input: T) -> AocResult<Flag> {
        let input = {
            let mut input = Input::from_str(input.as_ref())?.into_inner();
            input.sort_by_key(|num| Reverse(*num));
            input
        };
        let top: Calories = input[0..3].iter().copied().sum();

        Ok(Flag::new(top.into_inner() as i64))
    }
}
