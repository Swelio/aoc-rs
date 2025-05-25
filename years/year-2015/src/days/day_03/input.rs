use aoc_core::{
    archetypes::DayIdentity,
    components::{Day, Year},
    ports::Identity,
};

use super::components::NextMove;

#[derive(Debug, Clone, derive_more::IntoIterator)]
pub struct Input(#[into_iterator(owned, ref)] Vec<NextMove>);

impl Input {
    pub(super) fn new(moves: Vec<NextMove>) -> Self {
        Self(moves)
    }
}

impl Identity for Input {
    fn get_identity() -> aoc_core::archetypes::DayIdentity {
        DayIdentity::new(Year::new(2015), Day::try_new(3).expect("must be valid"))
    }
}
