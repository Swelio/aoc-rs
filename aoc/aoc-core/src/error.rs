pub type SantaResult<T> = Result<T, SantaError>;
pub type ParsingResult<T> = Result<T, ParsingError>;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SantaError {
    #[error("provided value is zero")]
    CannotBeZero,
    #[error("input is empty")]
    EmptyInput,
    #[error("solution is empty")]
    EmptySolution,
    #[error(transparent)]
    Parsing(#[from] ParsingError),
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{0}")]
pub struct ParsingError(String);

impl<S: AsRef<str>> From<S> for ParsingError {
    fn from(err: S) -> Self {
        Self(err.as_ref().to_string())
    }
}
