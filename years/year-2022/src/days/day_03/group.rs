pub mod rucksack;

use std::str::FromStr;

use aoc_traits::error::AocError;
use nutype::nutype;
use winnow::{
    ascii::{multispace0, multispace1},
    combinator::{separated, terminated},
    PResult, Parser,
};

use self::rucksack::{parse_rucksack, Rucksack};

#[nutype(derive(Debug, AsRef))]
pub struct Group(Vec<Rucksack>);

impl FromStr for Group {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(parse_rucksacks_group.parse(input)?)
    }
}

fn parse_rucksacks_group(input: &mut &str) -> PResult<Group> {
    let parser = separated(1.., parse_rucksack, multispace1);
    let mut parser = terminated(parser, multispace0);

    let rucksacks: Vec<_> = parser.parse_next(input)?;

    Ok(Group::new(rucksacks))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = include_str!("input.txt");
        let group = Group::from_str(input).unwrap();

        assert_eq!(group.as_ref().len(), 6);
    }
}
