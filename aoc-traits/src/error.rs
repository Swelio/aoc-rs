use std::io;

use crate::challenge::{Identity, IdentityError};

pub type AocResult<T> = std::result::Result<T, AocError>;

#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum AocError {
    #[error(transparent)]
    Identity(#[from] IdentityError),
    #[error("{0}")]
    FileError(String),
    #[error("solver is not implemented for challenge of year {} day {} part {}", .0.year(), .0.day(), .0.part())]
    NotImplemented(Identity),
}

impl From<io::Error> for AocError {
    fn from(err: io::Error) -> Self {
        Self::FileError(err.to_string())
    }
}
