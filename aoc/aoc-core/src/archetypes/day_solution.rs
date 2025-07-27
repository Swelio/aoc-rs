use super::DayIdentity;

#[derive(Debug, Clone, PartialEq, Eq, derive_more::Constructor, derive_more::Into)]
#[into((DayIdentity, S))]
pub struct DaySolution<S> {
    identity: DayIdentity,
    solutions: S,
}
