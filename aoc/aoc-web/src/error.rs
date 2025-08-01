use aoc_core::SantaError;

pub type WebResult<T> = Result<T, WebError>;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum WebError {
    #[error(transparent)]
    Santa(#[from] SantaError),
}
