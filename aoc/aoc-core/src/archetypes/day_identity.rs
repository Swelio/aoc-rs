use crate::components::{Day, Year};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::Constructor,
    derive_more::Into,
)]
#[into((Year, Day))]
pub struct DayIdentity {
    year: Year,
    day: Day,
}
