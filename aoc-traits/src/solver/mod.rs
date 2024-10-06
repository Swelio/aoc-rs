pub(crate) mod raw_input;
pub(crate) mod solution;

pub use raw_input::ChallengeRawInput;
pub use solution::Solution;

/// Challenge solving trait according to the identity of a challenge.
pub trait Solver<Year, Day, Part> {
    /// Solve the challenge part given the input.
    fn solve<Input: AsRef<str>>(&self, input: Input) -> Solution;
}
