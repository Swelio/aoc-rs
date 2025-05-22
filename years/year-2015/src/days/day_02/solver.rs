use aoc_core::{
    SantaResult,
    components::{Solution, parts::Part1},
    ports::Challenge,
};

use crate::days::day_02::components::Cuboid;

use super::input::Input;

impl Challenge<Solution<Part1>> for Input {
    fn solve(&self) -> SantaResult<Solution<Part1>> {
        let total_square: i32 = self
            .into_iter()
            .copied()
            .map(
                |Cuboid {
                     length,
                     width,
                     height,
                 }| {
                    let surfaces = [(length * width), (width * height), (height * length)];
                    let bonus = surfaces.into_iter().min().unwrap_or_default();

                    surfaces.into_iter().map(|side| 2 * side).sum::<i32>() + bonus
                },
            )
            .sum();

        Solution::try_new(total_square)
    }
}
