use crate::{
    archetypes::DaySolution,
    components::{
        Solution,
        parts::{Part1, Part2},
    },
    views::{IdentityView, SolutionPart},
};

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct SolutionView {
    pub identity: IdentityView,
    pub solutions: Vec<SolutionPart>,
}

impl From<DaySolution<(Solution<Part1>, Solution<Part2>)>> for SolutionView {
    fn from(domain: DaySolution<(Solution<Part1>, Solution<Part2>)>) -> Self {
        let (identity, solutions) = domain.into();

        let identity = identity.into();
        let solutions = vec![solutions.0.into(), solutions.1.into()];

        Self {
            identity,
            solutions,
        }
    }
}
