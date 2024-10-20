use std::str::FromStr;

use aoc_traits::error::AocError;
use winnow::{combinator::alt, PResult, Parser};

use super::{move_enum::Move, round::Opponent};

#[derive(Debug, Clone, Copy)]
pub enum OpponentDecision {
    A,
    B,
    C,
}

impl From<OpponentDecision> for Opponent {
    fn from(decision: OpponentDecision) -> Self {
        let selected = match decision {
            OpponentDecision::A => Move::Rock,
            OpponentDecision::B => Move::Paper,
            OpponentDecision::C => Move::Scissors,
        };

        Self::new(selected)
    }
}

impl FromStr for OpponentDecision {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(parse_opponent_decision.parse(input)?)
    }
}

pub fn parse_opponent_decision(input: &mut &str) -> PResult<OpponentDecision> {
    let mut parser = alt(('A', 'B', 'C'));
    let symbol = parser.parse_next(input)?;

    let decision = match symbol {
        'A' => OpponentDecision::A,
        'B' => OpponentDecision::B,
        'C' => OpponentDecision::C,
        _ => unreachable!("parser captures only valid chars"),
    };

    Ok(decision)
}
