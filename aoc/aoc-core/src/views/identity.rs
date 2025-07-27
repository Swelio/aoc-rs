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
    pub year: i32,
    pub day: u8,
}

impl From<DayIdentity> for IdentityView {
    fn from(domain: DayIdentity) -> Self {
        let (year, day) = domain.into();

        Self {
            year: year.into(),
            day: day.into(),
        }
    }
}
