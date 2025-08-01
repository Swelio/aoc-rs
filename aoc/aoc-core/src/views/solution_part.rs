use crate::components::Solution;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Constructor,
    derive_more::AsRef,
    derive_more::Display,
)]
pub struct SolutionPart(String);

impl<P> From<Solution<P>> for SolutionPart {
    fn from(value: Solution<P>) -> Self {
        Self(value.into())
    }
}
