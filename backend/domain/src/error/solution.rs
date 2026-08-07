use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SolutionValidationError {
    #[error("solution title cannot be blank")]
    BlankTitle,

    #[error("solution title must be {max} characters or fewer")]
    TitleTooLong { max: usize },

    #[error("solution body cannot be blank")]
    BlankBody,

    #[error("solution body must be {max} characters or fewer")]
    BodyTooLong { max: usize },

    #[error("comment body cannot be blank")]
    BlankCommentBody,

    #[error("comment body must be {max} characters or fewer")]
    CommentBodyTooLong { max: usize },

    #[error("submit URL must be {max} characters or fewer")]
    SubmitUrlTooLong { max: usize },

    #[error("submit URL must use http or https")]
    InvalidSubmitUrl,

    #[error("tags must be {max} or fewer")]
    TooManyTags { max: usize },

    #[error("tag must be {max} characters or fewer")]
    TagTooLong { max: usize },
}
