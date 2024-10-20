use std::str::FromStr;

use aoc_traits::prelude::*;

use crate::solver::Year2022Solver;

use super::models::{
    game::Game,
    move_enum::Move,
    player_decision::PlayerDecision,
    round::{Player, Round},
    round_decision::RoundDecision,
    score::Score,
};

impl Solver<Year2022, Day02, Part01> for Year2022Solver {
    fn solve<T: AsRef<str>>(&self, input: T) -> AocResult<Flag> {
        let game = Game::from_str(input.as_ref())?;
        let score: Score = game
            .into_iter()
            .map(Round::<Part01>::from)
            .map(|round| round.play())
            .sum();

        Ok(Flag::new(score.into_inner()))
    }
}

impl From<RoundDecision> for Round<Part01> {
    fn from(decision: RoundDecision) -> Self {
        let RoundDecision { player, opponent } = decision;
        let player = convert_player(player);

        Self::new(player, opponent.into())
    }
}

fn convert_player(decision: PlayerDecision) -> Player {
    let selected = match decision {
        PlayerDecision::X => Move::Rock,
        PlayerDecision::Y => Move::Paper,
        PlayerDecision::Z => Move::Scissors,
    };
    Player::new(selected)
}
