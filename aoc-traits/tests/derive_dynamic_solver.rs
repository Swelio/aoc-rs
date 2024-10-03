use aoc_traits::{
    days::Day01,
    parts::{Part01, Part02},
    solver::{Solution, Solver},
    years::Year2015,
};

#[derive(aoc_traits::DynamicSolver)]
#[solve(year(year = 2015, first_day = 1, last_day = 1))]
struct MockSolver;

impl Solver<Year2015, Day01, Part01> for MockSolver {
    fn solve<Input: AsRef<str>>(&self, _input: Input) -> Solution {
        todo!()
    }
}

impl Solver<Year2015, Day01, Part02> for MockSolver {
    fn solve<Input: AsRef<str>>(&self, _input: Input) -> Solution {
        todo!()
    }
}
