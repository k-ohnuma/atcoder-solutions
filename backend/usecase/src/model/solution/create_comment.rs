use domain::model::{solution::id::SolutionId, user::id::UserId};
use validator::{Validate, ValidationError};

#[derive(Validate)]
pub struct CreateCommentInput {
    pub user_id: UserId,
    pub solution_id: SolutionId,
    #[validate(length(min = 1, max = 2000), custom(function = "validate_not_blank"))]
    pub body_md: String,
}

fn validate_not_blank(value: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        return Err(ValidationError::new("blank"));
    }
    Ok(())
}
