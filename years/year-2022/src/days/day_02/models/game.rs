use std::{iter::Copied, slice, str::FromStr};

use aoc_traits::error::AocError;
use nutype::nutype;
use winnow::{
    ascii::{multispace0, multispace1},
    combinator::{separated, terminated},
    PResult, Parser,
};

use super::round_decision::{parse_round_decision, RoundDecision};

#[nutype(derive(Debug, AsRef))]
pub struct Game(Vec<RoundDecision>);

impl<'a> IntoIterator for &'a Game {
    type Item = RoundDecision;
    type IntoIter = Copied<slice::Iter<'a, Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().iter().copied()
    }
}

impl FromStr for Game {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(parse_game.parse(input)?)
    }
}

pub fn parse_game(input: &mut &str) -> PResult<Game> {
    let parser = separated(1.., parse_round_decision, multispace1);
    let mut parser = terminated(parser, multispace0);
    let rounds = parser.parse_next(input)?;

    Ok(Game::new(rounds))
}
