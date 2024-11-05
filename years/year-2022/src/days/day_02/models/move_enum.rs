use std::cmp::Ordering;

use super::score::Score;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Move {
    Paper,
    Rock,
    Scissors,
}

impl Move {
    pub fn score(self) -> Score {
        match self {
            Move::Rock => Score::new(1),
            Move::Paper => Score::new(2),
            Move::Scissors => Score::new(3),
        }
    }

    /// Get losing move against self
    pub fn losing(self) -> Self {
        match self {
            Self::Paper => Self::Rock,
            Self::Rock => Self::Scissors,
            Self::Scissors => Self::Paper,
        }
    }

    /// Get winning move against self
    pub fn winning(self) -> Self {
        match self {
            Self::Paper => Self::Scissors,
            Self::Rock => Self::Paper,
            Self::Scissors => Self::Rock,
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Move::Paper, Move::Paper)
            | (Move::Rock, Move::Rock)
            | (Move::Scissors, Move::Scissors) => Ordering::Equal,
            (Move::Paper, Move::Rock)
            | (Move::Rock, Move::Scissors)
            | (Move::Scissors, Move::Paper) => Ordering::Greater,
            (Move::Paper, Move::Scissors)
            | (Move::Rock, Move::Paper)
            | (Move::Scissors, Move::Rock) => Ordering::Less,
        }
    }
}
