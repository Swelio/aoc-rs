use std::{ops::RangeInclusive, str::FromStr};

use aoc_traits::error::AocError;
use nutype::nutype;
use winnow::{
    ascii::{digit1, multispace0, multispace1},
    combinator::{separated, separated_pair, terminated},
    PResult, Parser,
};

#[nutype(derive(Debug, AsRef))]
pub struct Camp(Vec<Pair>);

impl FromStr for Camp {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(parse_camp.parse(input)?)
    }
}

fn parse_camp(input: &mut &str) -> PResult<Camp> {
    let parser = separated(1.., parse_pair, multispace1);
    let pairs = terminated(parser, multispace0).parse_next(input)?;

    Ok(Camp::new(pairs))
}

#[derive(Debug)]
pub struct Pair(Assignment, Assignment);

impl Pair {
    pub fn new(first: Assignment, second: Assignment) -> Self {
        Self(first, second)
    }

    pub fn is_self_contained(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    pub fn is_self_overlapping(&self) -> bool {
        self.0.overlap(&self.1)
    }
}

impl FromStr for Pair {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(parse_pair.parse(input)?)
    }
}

fn parse_pair(input: &mut &str) -> PResult<Pair> {
    let (first, second) =
        separated_pair(parse_assignment, ',', parse_assignment).parse_next(input)?;

    Ok(Pair::new(first, second))
}

#[nutype(derive(Debug, Clone, AsRef))]
pub struct Assignment(RangeInclusive<Section>);

impl Assignment {
    pub fn contains(&self, other: &Self) -> bool {
        self.as_ref().contains(other.as_ref().start())
            && self.as_ref().contains(other.as_ref().end())
    }

    pub fn overlap(&self, other: &Self) -> bool {
        self.as_ref().contains(other.as_ref().start())
            || self.as_ref().contains(other.as_ref().end())
            || other.as_ref().contains(self.as_ref().start())
            || other.as_ref().contains(self.as_ref().end())
    }
}

impl FromStr for Assignment {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(parse_assignment.parse(input)?)
    }
}

fn parse_assignment(input: &mut &str) -> PResult<Assignment> {
    let (start, end) =
        separated_pair(digit1.parse_to(), '-', digit1.parse_to()).parse_next(input)?;
    let range = start..=end;

    Ok(Assignment::new(range))
}

#[nutype(derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord))]
pub struct Section(u32);

impl FromStr for Section {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(parse_section.parse(input)?)
    }
}

fn parse_section(input: &mut &str) -> PResult<Section> {
    let inner = digit1.parse_to().parse_next(input)?;
    Ok(Section::new(inner))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "2-4,6-8";
        let _ = Pair::from_str(input).unwrap();
    }
}
