#![allow(dead_code)]

use aoc_traits::{
    challenge::Flag,
    days::Day01,
    error::AocResult,
    parts::{Part01, Part02},
    years::Year2015,
    Solver,
};

#[derive(aoc_traits::DynamicSolver)]
#[solve(year(year = 2015, first_day = 1, last_day = 1))]
struct MockSolver;

impl Solver<Year2015, Day01, Part01> for MockSolver {
    fn solve<Input: AsRef<str>>(&self, _input: Input) -> AocResult<Flag> {
        todo!()
    }
}

impl Solver<Year2015, Day01, Part02> for MockSolver {
    fn solve<Input: AsRef<str>>(&self, _input: Input) -> AocResult<Flag> {
        todo!()
    }
}
