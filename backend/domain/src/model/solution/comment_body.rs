use crate::error::solution::SolutionValidationError;

const MAX_COMMENT_BODY_LENGTH: usize = 2_000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommentBody(String);

impl CommentBody {
    pub fn try_new(value: String) -> Result<Self, SolutionValidationError> {
        if value.trim().is_empty() {
            return Err(SolutionValidationError::BlankCommentBody);
        }
        if value.chars().count() > MAX_COMMENT_BODY_LENGTH {
            return Err(SolutionValidationError::CommentBodyTooLong {
                max: MAX_COMMENT_BODY_LENGTH,
            });
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::error::solution::SolutionValidationError;

    use super::CommentBody;

    #[test]
    fn preserves_valid_body() {
        let body = CommentBody::try_new(" comment ".to_string()).expect("valid comment body");

        assert_eq!(body.as_str(), " comment ");
    }

    #[test]
    fn rejects_blank_or_too_long_body() {
        assert_eq!(
            CommentBody::try_new("   ".to_string()),
            Err(SolutionValidationError::BlankCommentBody)
        );
        assert_eq!(
            CommentBody::try_new("a".repeat(2_001)),
            Err(SolutionValidationError::CommentBodyTooLong { max: 2_000 })
        );
    }
}
