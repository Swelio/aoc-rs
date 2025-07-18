use aoc_core::{
    archetypes::DayIdentity,
    components::{Day, Year},
    ports::Identity,
};

use super::components::Line;

#[derive(Debug, Clone, derive_more::IntoIterator)]
pub struct Input(#[into_iterator(owned, ref)] Vec<Line>);

impl Identity for Input {
    fn get_identity() -> aoc_core::archetypes::DayIdentity {
        DayIdentity::new(
            Year::try_new(2015).expect("must be valid"),
            Day::try_new(5).expect("must be valid"),
        )
    }
}

impl Input {
    pub(crate) fn new(lines: Vec<Line>) -> Self {
        Self(lines)
    }
}
