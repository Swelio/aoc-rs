use crate::solver::solution::Solution;

use super::{identity::Identity, request::ChallengeRequest};

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
}
