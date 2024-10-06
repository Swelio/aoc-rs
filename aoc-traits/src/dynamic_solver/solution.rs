use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

use crate::solver::Solution;

use super::{identity::Identity, request::ChallengeRequest};

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct ChallengeSolution {
    id: Identity,
    solution: Solution,
}

impl ChallengeSolution {
    pub fn new(request: &ChallengeRequest, solution: Solution) -> Self {
        Self {
            id: request.id(),
            solution,
        }
    }

    pub fn id(&self) -> Identity {
        self.id
    }

    pub fn solution(&self) -> &Solution {
        &self.solution
    }
}

impl Display for ChallengeSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.id, self.solution)
    }
}

impl PartialOrd for ChallengeSolution {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ChallengeSolution {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
