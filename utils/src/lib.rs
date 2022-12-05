#![deny(clippy::all)]

use std::fmt::{Display, Formatter};
use std::io::Read;

/// Code solution of a day.
pub trait CodeSolution {
    /// Run the solution with the provided input.
    /// As no output is expected, it must display the result by itself.
    fn run<I>(input: I) -> Result<(), Box<dyn std::error::Error>>
    where
        I: Read;
}

/// Generic error which may occur during puzzle resolution.
#[derive(Clone, Debug)]
pub enum SantaError {
    InvalidInput(String),
}

impl std::error::Error for SantaError {}

impl Display for SantaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput(msg) => write!(f, "Invalid input. Error: {}", msg),
        }
    }
}
