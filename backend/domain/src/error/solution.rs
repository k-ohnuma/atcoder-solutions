use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SolutionValidationError {
    #[error("tags must be {max} or fewer")]
    TooManyTags { max: usize },

    #[error("tag must be {max} characters or fewer")]
    TagTooLong { max: usize },
}
