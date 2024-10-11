use aoc_traits::error::AocError;
use winnow::error::{ContextError, ParseError};

#[derive(Debug, thiserror::Error)]
pub enum SolverError {
    #[error(transparent)]
    Aoc(#[from] AocError),
    #[error("argument parsing failed: {0}")]
    ArgumentParsing(String),
}

impl From<ParseError<&str, ContextError>> for SolverError {
    fn from(err: ParseError<&str, ContextError>) -> Self {
        Self::ArgumentParsing(err.to_string())
    }
}
