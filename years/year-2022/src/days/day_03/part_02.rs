use std::{collections::HashSet, str::FromStr};

use aoc_traits::prelude::*;

use crate::Year2022Solver;

use super::group::{rucksack::item::Priority, Group};

impl Solver<Year2022, Day03, Part02> for Year2022Solver {
    fn solve<T: AsRef<str>>(&self, input: T) -> AocResult<Flag> {
        let group = Group::from_str(input.as_ref())?;

        let total = group
            .into_inner()
            .chunks_exact(3)
            .flat_map(|chunk| {
                chunk
                    .iter()
                    .map(|rucksack| {
                        rucksack
                            .as_ref()
                            .iter()
                            .flat_map(|compartment| compartment.as_ref().iter().copied())
                            .collect::<HashSet<_>>()
                    })
                    .reduce(|mut intersection, current| {
                        intersection.retain(|item| current.contains(item));
                        intersection
                    })
                    .into_iter()
                    .flat_map(|intersection| intersection.into_iter().map(|item| item.priority()))
            })
            .sum::<Priority>();

        Ok(Flag::from(total.into_inner()))
    }
}
