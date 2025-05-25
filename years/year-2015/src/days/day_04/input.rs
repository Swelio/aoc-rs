use aoc_core::{
    archetypes::DayIdentity,
    components::{Day, Year},
    ports::Identity,
};

#[derive(Debug, Clone, Copy)]
pub struct Input;

impl Identity for Input {
    fn get_identity() -> aoc_core::archetypes::DayIdentity {
        DayIdentity::new(Year::new(2015), Day::try_new(4).expect("must be valid"))
    }
}
