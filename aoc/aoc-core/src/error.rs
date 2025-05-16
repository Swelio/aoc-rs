pub type SantaResult<T> = Result<T, SantaError>;

#[derive(Debug, thiserror::Error)]
pub enum SantaError {
    #[error("input is empty")]
    EmptyInput,
    #[error("parsing error: {0}")]
    ParsingError(#[source] anyhow::Error),
}
