use super::DayIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, derive_more::Constructor)]
pub struct DaySolution<S> {
    #[serde(flatten)]
    pub identity: DayIdentity,
    pub solutions: S,
}
