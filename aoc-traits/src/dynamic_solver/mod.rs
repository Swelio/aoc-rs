pub mod identity;
pub mod request;
pub mod solution;

use request::ChallengeRequest;
use solution::ChallengeSolution;

pub trait DynamicSolver {
    fn resolve(&self, request: ChallengeRequest) -> ChallengeSolution;
}
