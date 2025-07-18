use super::DayIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::Constructor)]
pub struct DaySolution<S> {
    pub identity: DayIdentity,
    pub solutions: S,
}
