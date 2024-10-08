use crate::{challenge::Flag, error::AocResult};

/// Challenge solving trait according to the identity of a challenge.
pub trait Solver<Year, Day, Part> {
    /// Solve the challenge part given the input.
    fn solve<Input: AsRef<str>>(&self, input: Input) -> AocResult<Flag>;
}
