pub mod compartment;
pub mod item;

use std::{collections::HashSet, str::FromStr};

use aoc_traits::error::AocError;
use nutype::nutype;
use winnow::{ascii, error::ContextError, PResult, Parser};

use self::{compartment::Compartment, item::Item};

#[nutype(derive(Debug, AsRef))]
pub struct Rucksack(Vec<Compartment>);

impl FromStr for Rucksack {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(parse_rucksack.parse(input)?)
    }
}

pub fn parse_rucksack(input: &mut &str) -> PResult<Rucksack> {
    let input = ascii::alpha1.parse_next(input)?;
    let middle = input.len() / 2;

    if input.len() % middle != 0 {
        return Err(winnow::error::ErrMode::Cut(ContextError::default()));
    }

    let compartments = input.chars().map(Item::new).fold(Vec::new(), {
        let mut total = 0;

        move |mut acc, item| -> Vec<HashSet<Item>> {
            match acc.last_mut().filter(|_| total % middle != 0) {
                Some(current) => {
                    current.insert(item);
                }
                None => {
                    acc.push([item].into());
                }
            };

            total += 1;
            acc
        }
    });
    let compartments = compartments.into_iter().map(Compartment::new).collect();

    Ok(Rucksack::new(compartments))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let rucksack = Rucksack::from_str(input).unwrap();
        let common_items = rucksack
            .as_ref()
            .iter()
            .cloned()
            .reduce(|union, compartment| union.intersection(&compartment));

        assert_eq!(rucksack.as_ref().len(), 2);
        assert_eq!(
            common_items,
            Some(Compartment::new([Item::new('p')].into()))
        );
    }
}
