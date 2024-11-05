use winnow::error::{ContextError, ParseError};

#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
pub enum IdentityError {
    #[error("parsing failed: {0}")]
    Parsing(String),
    #[error("validation error(s): {0:?}")]
    Validation(Vec<String>),
}

impl From<ParseError<&str, ContextError>> for IdentityError {
    fn from(err: ParseError<&str, ContextError>) -> Self {
        Self::Parsing(err.to_string())
    }
}
