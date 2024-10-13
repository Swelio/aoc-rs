use std::str::FromStr;

use aoc_traits::prelude::*;

use crate::solver::Year2022Solver;

use super::input::Input;

impl Solver<Year2022, Day01, Part01> for Year2022Solver {
    fn solve<T: AsRef<str>>(&self, input: T) -> AocResult<Flag> {
        let input = Input::from_str(input.as_ref())?;
        let max_calories = input.as_ref().iter().max().ok_or(AocError::EmptyResult)?;

        Ok(Flag::new(max_calories.into_inner() as i64))
    }
}
