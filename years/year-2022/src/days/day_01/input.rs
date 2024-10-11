use std::{iter::Sum, ops::Add, str::FromStr};

use aoc_traits::error::AocError;
use nutype::nutype;
use winnow::{
    ascii::{digit1, line_ending, multispace0},
    combinator::{separated, terminated},
    PResult, Parser,
};

#[nutype(derive(Debug, AsRef))]
pub struct Input(Vec<Calories>);

impl FromStr for Input {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(input_parser.parse(input)?)
    }
}

fn input_parser(input: &mut &str) -> PResult<Input> {
    let parser = separated(1.., total_calories_parser, (line_ending, line_ending));
    let mut parser = terminated(parser, multispace0);
    let packages: Vec<_> = parser.parse_next(input)?;

    Ok(Input::new(packages))
}

#[nutype(
    derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord),
    default = 0
)]
pub struct Calories(u32);

impl Add for Calories {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.into_inner() + rhs.into_inner())
    }
}

impl Sum for Calories {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|total, num| total + num).unwrap_or_default()
    }
}

impl FromStr for Calories {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(total_calories_parser.parse(input)?)
    }
}

fn total_calories_parser(input: &mut &str) -> PResult<Calories> {
    let calories = digit1.parse_to().map(Calories::new);
    let mut package = separated(1.., calories, line_ending);
    let collected_calories: Vec<_> = package.parse_next(input)?;

    Ok(collected_calories.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("tests/input.txt");

    #[test]
    fn parse_package() {
        let input = r#"1000
2000
3000"#;
        let calories = Calories::from_str(input).unwrap();
        assert_eq!(calories, Calories::new(6000));
    }

    #[test]
    fn parse_packages() {
        let expected = Input::new(vec![
            Calories::new(6000),
            Calories::new(4000),
            Calories::new(11000),
            Calories::new(24000),
            Calories::new(10000),
        ]);
        let result = Input::from_str(INPUT).unwrap();

        assert_eq!(result.as_ref(), expected.as_ref());
    }
}
