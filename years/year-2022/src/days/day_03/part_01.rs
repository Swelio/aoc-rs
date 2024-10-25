use std::str::FromStr;

use aoc_traits::prelude::*;

use crate::{
    days::day_03::group::{rucksack::item::Priority, Group},
    Year2022Solver,
};

use super::group::rucksack::intersectable::Intersectable;

impl Solver<Year2022, Day03, Part01> for Year2022Solver {
    fn solve<T: AsRef<str>>(&self, input: T) -> AocResult<Flag> {
        let group = Group::from_str(input.as_ref())?;
        let total: Priority = group
            .into_inner()
            .into_iter()
            .map(|rucksack| {
                let intersection = rucksack.as_ref().iter().intersect();

                intersection
                    .into_inner()
                    .into_iter()
                    .map(|item| item.priority())
                    .sum::<Priority>()
            })
            .sum();

        Ok(Flag::from(total.into_inner()))
    }
}
