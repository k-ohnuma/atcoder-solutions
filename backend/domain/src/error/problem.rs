use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ProblemValidationError {
    #[error("problemId cannot be empty")]
    BlankId,
}
