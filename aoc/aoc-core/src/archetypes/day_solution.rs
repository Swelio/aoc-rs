use super::DayIdentity;

#[derive(Debug, PartialEq, Eq, serde::Serialize, derive_more::Constructor)]
pub struct DaySolution<S> {
    #[serde(flatten)]
    identity: DayIdentity,
    solutions: S,
}
