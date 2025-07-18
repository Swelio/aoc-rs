use crate::archetypes::DayIdentity;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
)]
#[display("{year}/{day:02}")]
pub struct IdentityView {
    year: i32,
    day: u8,
}

impl From<DayIdentity> for IdentityView {
    fn from(domain: DayIdentity) -> Self {
        Self {
            year: domain.year.into(),
            day: domain.day.into(),
        }
    }
}
