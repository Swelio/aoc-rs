use crate::{
    archetypes::DaySolution,
    components::{
        Solution,
        parts::{Part1, Part2},
    },
    views::IdentityView,
};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SolutionView {
    identity: IdentityView,
    solutions: Vec<String>,
}

impl From<DaySolution<(Solution<Part1>, Solution<Part2>)>> for SolutionView {
    fn from(domain: DaySolution<(Solution<Part1>, Solution<Part2>)>) -> Self {
        let identity = domain.identity.into();
        let (part_one, part_two) = domain.solutions;
        let solutions = vec![part_one.into(), part_two.into()];

        Self {
            identity,
            solutions,
        }
    }
}
