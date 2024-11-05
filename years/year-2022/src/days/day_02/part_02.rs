use std::str::FromStr;

use aoc_traits::prelude::*;

use crate::solver::Year2022Solver;

use super::models::{
    game::Game,
    player_decision::PlayerDecision,
    round::{Opponent, Player, Round},
    round_decision::RoundDecision,
    score::Score,
};

impl Solver<Year2022, Day02, Part02> for Year2022Solver {
    fn solve<T: AsRef<str>>(&self, input: T) -> AocResult<Flag> {
        let game = Game::from_str(input.as_ref())?;
        let score: Score = game
            .into_iter()
            .map(Round::<Part02>::from)
            .map(|round| round.play())
            .sum();

        Ok(Flag::from(score.into_inner()))
    }
}

impl From<RoundDecision> for Round<Part02> {
    fn from(decision: RoundDecision) -> Self {
        let RoundDecision { player, opponent } = decision;
        let player = Strategy::from(player);
        let opponent = Opponent::from(opponent);

        let player = match player {
            Strategy::Draw => opponent.as_ref().to_owned(),
            Strategy::Lose => opponent.as_ref().losing(),
            Strategy::Win => opponent.as_ref().winning(),
        };

        Self::new(Player::new(player), opponent)
    }
}

#[derive(Debug)]
enum Strategy {
    Draw,
    Lose,
    Win,
}

impl From<PlayerDecision> for Strategy {
    fn from(decision: PlayerDecision) -> Self {
        match decision {
            PlayerDecision::X => Self::Lose,
            PlayerDecision::Y => Self::Draw,
            PlayerDecision::Z => Self::Win,
        }
    }
}
