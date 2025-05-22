use aoc_core::{
    SantaResult,
    components::{
        Solution,
        parts::{Part1, Part2},
    },
    ports::Challenge,
};

use crate::days::day_02::components::Cuboid;

use super::input::Input;

impl Challenge<Solution<Part1>> for Input {
    fn solve(&self) -> SantaResult<Solution<Part1>> {
        let total_square: u32 = self
            .into_iter()
            .copied()
            .map(
                |Cuboid {
                     length,
                     width,
                     height,
                 }| {
                    let surfaces = [(length * width), (width * height), (height * length)];
                    let bonus = surfaces.into_iter().min().expect("cuboid has dimensions");

                    surfaces.into_iter().map(|side| 2 * side).sum::<u32>() + bonus
                },
            )
            .sum();

        Solution::try_new(total_square)
    }
}

impl Challenge<Solution<Part2>> for Input {
    fn solve(&self) -> SantaResult<Solution<Part2>> {
        let total_ribbon: u32 = self
            .into_iter()
            .copied()
            .map(
                |Cuboid {
                     length,
                     width,
                     height,
                 }| {
                    let dimensions = {
                        let mut dimensions = [length, width, height];
                        dimensions.sort();
                        dimensions
                    };

                    let perimeter = {
                        let lowest_sides = &dimensions[0..2];
                        lowest_sides.iter().chain(lowest_sides.iter()).sum::<u32>()
                    };

                    let bow = dimensions
                        .into_iter()
                        .reduce(|acc, current| acc * current)
                        .expect("cuboid has dimensions");

                    perimeter + bow
                },
            )
            .sum();

        Solution::try_new(total_ribbon)
    }
}
