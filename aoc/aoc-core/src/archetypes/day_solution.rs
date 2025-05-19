#[derive(Debug, PartialEq, Eq, serde::Serialize, derive_more::Constructor)]
pub struct DaySolution<S> {
    year: i16,
    day: u8,
    solutions: S,
}
