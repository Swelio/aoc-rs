#![deny(clippy::all)]

use std::fmt::{Display, Formatter};
use std::io::{Read, Seek};
use std::sync::Once;

static LOG_INIT: Once = Once::new();

pub fn setup_log() {
    LOG_INIT.call_once(env_logger::init);
}

/// Code solution of a day.
pub trait CodeSolution {
    /// Run the solution with the provided input.
    /// As no output is expected, it must display the result by itself.
    fn run<I>(input: I) -> Result<(), Box<dyn std::error::Error>>
    where
        I: Read + Seek;
}

/// Generic error which may occur during puzzle resolution.
#[derive(Clone, Debug)]
pub enum SantaError {
    InvalidInput(String),
    WrongProcedure(String),
}

impl std::error::Error for SantaError {}

impl Display for SantaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput(msg) => write!(f, "Invalid input. Error: {}", msg),
            Self::WrongProcedure(msg) => write!(f, "Wrong procedure. Error: {}", msg),
        }
    }
}
