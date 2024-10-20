use std::str::FromStr;

use aoc_traits::error::AocError;
use winnow::{seq, PResult, Parser};

use crate::days::day_02::models::{
    opponent_decision::parse_opponent_decision, player_decision::parse_player_decision,
};

use super::{opponent_decision::OpponentDecision, player_decision::PlayerDecision};

#[derive(Debug, Clone, Copy)]
pub struct RoundDecision {
    pub player: PlayerDecision,
    pub opponent: OpponentDecision,
}

impl FromStr for RoundDecision {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(parse_round_decision.parse(input)?)
    }
}

pub fn parse_round_decision(input: &mut &str) -> PResult<RoundDecision> {
    seq! {
        RoundDecision {
            opponent: parse_opponent_decision,
            _: ' ',
            player: parse_player_decision
        }
    }
    .parse_next(input)
}
