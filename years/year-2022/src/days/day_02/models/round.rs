use std::{cmp::Ordering, marker::PhantomData};

use nutype::nutype;

use super::{move_enum::Move, score::Score};

#[nutype(derive(Debug, AsRef))]
pub struct Opponent(Move);

#[nutype(derive(Debug, AsRef))]
pub struct Player(Move);

pub struct Round<P> {
    part: PhantomData<P>,
    player: Player,
    opponent: Opponent,
}

impl<P> Round<P> {
    pub fn new(player: Player, opponent: Opponent) -> Self {
        Self {
            part: PhantomData,
            player,
            opponent,
        }
    }

    pub fn play(self) -> Score {
        let move_score = self.player.as_ref().score();
        let outcome_score = self.compute_outcome();

        move_score + outcome_score
    }

    fn compute_outcome(&self) -> Score {
        match self.player.as_ref().cmp(self.opponent.as_ref()) {
            Ordering::Less => Score::new(0),
            Ordering::Equal => Score::new(3),
            Ordering::Greater => Score::new(6),
        }
    }
}
