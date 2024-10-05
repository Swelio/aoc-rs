pub use identity::{Day, DayError, Identity, Part, PartError, Year, YearError};
pub use request::ChallengeRequest;
pub use solution::ChallengeSolution;

mod identity;
mod request;
mod solution;

use crate::error::AocResult;

pub trait DynamicSolver {
    fn resolve(&self, request: ChallengeRequest) -> AocResult<ChallengeSolution>;
}
