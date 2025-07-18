use aoc_core::{
    archetypes::DayIdentity,
    components::{Day, Year},
    ports::Identity,
};

use super::components::Cuboid;

#[derive(Debug, derive_more::Constructor, derive_more::IntoIterator)]
pub struct Input {
    #[into_iterator(owned, ref)]
    cuboids: Vec<Cuboid>,
}

impl Identity for Input {
    fn get_identity() -> aoc_core::archetypes::DayIdentity {
        DayIdentity::new(
            Year::try_new(2015).expect("must be valid"),
            Day::try_new(2).expect("must be valid"),
        )
    }
}
