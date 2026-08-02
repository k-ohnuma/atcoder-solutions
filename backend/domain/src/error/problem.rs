use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ProblemValidationError {
    #[error("contestCode cannot be empty")]
    BlankContestCode,
    #[error("problemId cannot be empty")]
    BlankId,
    #[error("problemIndex cannot be empty")]
    BlankIndex,
    #[error("problemTitle cannot be empty")]
    BlankTitle,
}
