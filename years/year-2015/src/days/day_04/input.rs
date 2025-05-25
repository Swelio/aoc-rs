use aoc_core::{
    archetypes::DayIdentity,
    components::{Day, Year},
    ports::Identity,
};

#[derive(Debug, Clone, derive_more::Constructor, derive_more::AsRef)]
#[as_ref(str, [u8], String)]
pub struct Input(String);

impl Identity for Input {
    fn get_identity() -> aoc_core::archetypes::DayIdentity {
        DayIdentity::new(Year::new(2015), Day::try_new(4).expect("must be valid"))
    }
}
