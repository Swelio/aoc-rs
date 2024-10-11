use std::io;

use winnow::error::{ContextError, ParseError};

use crate::challenge::{Identity, IdentityError};

pub type AocResult<T> = std::result::Result<T, AocError>;

#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum AocError {
    #[error("result is empty")]
    EmptyResult,
    #[error(transparent)]
    Identity(#[from] IdentityError),
    #[error("{0}")]
    File(String),
    #[error("solver is not implemented for challenge of year {} day {} part {}", .0.year(), .0.day(), .0.part())]
    NotImplemented(Identity),
    #[error("error occurred while parsing: {0}")]
    Parsing(String),
}

impl AocError {
    pub fn file(err: impl ToString) -> Self {
        Self::File(err.to_string())
    }

    pub fn parsing(err: impl ToString) -> Self {
        Self::Parsing(err.to_string())
    }
}

impl From<io::Error> for AocError {
    fn from(err: io::Error) -> Self {
        Self::file(err)
    }
}

impl From<ParseError<&str, ContextError>> for AocError {
    fn from(err: ParseError<&str, ContextError>) -> Self {
        Self::parsing(err)
    }
}
