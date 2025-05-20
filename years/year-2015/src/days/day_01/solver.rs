use aoc_core::{
    SantaResult,
    components::{
        Solution,
        parts::{Part1, Part2},
    },
    ports::Challenge,
};

use super::{components::Direction, input::Day01Input};

impl Challenge<Solution<Part1>> for Day01Input {
    fn solve(&self) -> SantaResult<Solution<Part1>> {
        Ok(self
            .into_iter()
            .map(|direction| match direction {
                Direction::Up => 1,
                Direction::Down => -1,
            })
            .sum::<i32>())
        .and_then(Solution::try_new)
    }
}

impl Challenge<Solution<Part2>> for Day01Input {
    fn solve(&self) -> SantaResult<Solution<Part2>> {
        todo!()
    }
}
