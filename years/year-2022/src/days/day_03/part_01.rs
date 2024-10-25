use std::str::FromStr;

use aoc_traits::prelude::*;

use crate::{
    days::day_03::group::{rucksack::item::Priority, Group},
    Year2022Solver,
};

impl Solver<Year2022, Day03, Part01> for Year2022Solver {
    fn solve<T: AsRef<str>>(&self, input: T) -> AocResult<Flag> {
        let group = Group::from_str(input.as_ref())?;
        let total: Priority = group
            .into_inner()
            .into_iter()
            .filter_map(|rucksack| {
                rucksack
                    .into_inner()
                    .into_iter()
                    .reduce(|intersection, current| intersection.intersection(&current))
                    .map(|intersection| {
                        let total: Priority = intersection
                            .into_inner()
                            .into_iter()
                            .map(|item| item.priority())
                            .sum();

                        total
                    })
            })
            .sum();

        Ok(Flag::new(total.into_inner()))
    }
}
