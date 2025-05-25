use crate::archetypes::DayIdentity;

pub trait Identity {
    fn get_identity() -> DayIdentity;
}
