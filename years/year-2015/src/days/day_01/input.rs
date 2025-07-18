use aoc_core::{
    archetypes::DayIdentity,
    components::{Day, Year},
    ports::Identity,
};

use super::components::Direction;

#[derive(Debug, derive_more::IntoIterator)]
pub struct Input {
    #[into_iterator(owned, ref)]
    directions: Vec<Direction>,
}

impl Input {
    pub(super) fn new(directions: Vec<Direction>) -> Self {
        Self { directions }
    }
}

impl Identity for Input {
    fn get_identity() -> aoc_core::archetypes::DayIdentity {
        DayIdentity::new(
            Year::try_new(2015).expect("must be valid"),
            Day::try_new(1).expect("must be valid"),
        )
    }
}
