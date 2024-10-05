use crate::dynamic_solver::Identity;

pub type AocResult<T> = std::result::Result<T, AocError>;

#[derive(Debug, thiserror::Error)]
pub enum AocError {
    #[error("solver is not implemented for challenge of year {} day {} part {}", .0.year(), .0.day(), .0.part())]
    NotImplemented(Identity),
}
