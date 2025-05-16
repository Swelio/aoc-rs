use aoc_core::{SantaResult, components::Solution, ports::Solver};

use crate::solver::Year2015Solver;

use super::{components::Direction, input::Day01Input};

impl Solver<Day01Input> for Year2015Solver {
    type Output = i32;

    fn solve(&self, input: &Day01Input) -> SantaResult<Solution<Day01Input, Self::Output>> {
        Ok(Solution::new(solve_part_01(input), 0))
    }
}

fn solve_part_01(input: &Day01Input) -> i32 {
    input
        .into_iter()
        .map(|direction| match direction {
            Direction::Up => 1,
            Direction::Down => -1,
        })
        .sum()
}
