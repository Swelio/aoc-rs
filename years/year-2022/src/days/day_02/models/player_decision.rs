use std::str::FromStr;

use aoc_traits::error::AocError;
use winnow::{combinator::alt, PResult, Parser};

#[derive(Debug, Clone, Copy)]
pub enum PlayerDecision {
    X,
    Y,
    Z,
}

impl FromStr for PlayerDecision {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(parse_player_decision.parse(input)?)
    }
}

pub fn parse_player_decision(input: &mut &str) -> PResult<PlayerDecision> {
    let mut parser = alt(('X', 'Y', 'Z'));
    let symbol = parser.parse_next(input)?;

    let decision = match symbol {
        'X' => PlayerDecision::X,
        'Y' => PlayerDecision::Y,
        'Z' => PlayerDecision::Z,
        _ => unreachable!("parser captures only valid chars"),
    };

    Ok(decision)
}
