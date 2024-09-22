pub mod identity;
pub(crate) mod request;
pub(crate) mod solution;

use solution::ChallengeSolution;

/// Challenge solving trait according to the identity of a challenge.
pub trait Solver<Identity> {
    /// Solve the challenge part given the input.
    fn solve<Input: AsRef<str>>(&self, input: Input) -> ChallengeSolution<Identity>;
}
