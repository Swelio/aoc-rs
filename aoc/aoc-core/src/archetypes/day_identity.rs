use crate::components::{Day, Year};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    serde::Serialize,
    derive_more::Constructor,
    derive_more::Display,
)]
#[display("{year}/{day}")]
pub struct DayIdentity {
    pub year: Year,
    pub day: Day,
}
