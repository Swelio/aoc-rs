pub use raw_input::ChallengeRawInput;
pub use solution::Solution;

mod raw_input;
mod solution;

use crate::error::AocResult;

/// Challenge solving trait according to the identity of a challenge.
pub trait Solver<Year, Day, Part> {
    /// Solve the challenge part given the input.
    fn solve<Input: AsRef<str>>(&self, input: Input) -> AocResult<Solution>;
}
