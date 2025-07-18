use crate::components::{Day, Year};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, derive_more::Constructor)]
pub struct DayIdentity {
    pub year: Year,
    pub day: Day,
}
