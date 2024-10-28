use std::str::FromStr;

use aoc_traits::error::AocError;
use nutype::nutype;
use winnow::{combinator::delimited, token::any, PResult, Parser};

#[nutype(derive(Debug, Clone, PartialEq, Eq))]
pub struct Cargo(Vec<Stack>);

#[nutype(derive(Debug, Clone, PartialEq, Eq))]
pub struct Stack(Vec<Crate>);

#[nutype(derive(Debug, Clone, Copy, PartialEq, Eq))]
pub struct Crate(char);

impl FromStr for Cargo {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(parse_cargo.parse(input)?)
    }
}

fn parse_cargo(input: &mut &str) -> PResult<Cargo> {
    todo!()
}

fn parse_crate(input: &mut &str) -> PResult<Crate> {
    let inner = delimited(
        '[',
        any.verify(|token: &char| token.is_ascii_uppercase()),
        ']',
    )
    .parse_next(input)?;

    Ok(Crate::new(inner))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn parsing_cargo() {
        let _ = Cargo::from_str(INPUT).unwrap();
    }
}
