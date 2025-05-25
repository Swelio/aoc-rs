use crate::components::{Day, Year};

#[derive(
    Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, derive_more::Constructor,
)]
pub struct DayIdentity {
    year: Year,
    day: Day,
}
