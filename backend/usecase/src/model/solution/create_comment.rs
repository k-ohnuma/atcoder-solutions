use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Validate)]
pub struct CreateCommentInput {
    #[validate(length(min = 1, max = 120), custom(function = "validate_not_blank"))]
    pub user_id: String,
    pub solution_id: Uuid,
    #[validate(length(min = 1, max = 2000), custom(function = "validate_not_blank"))]
    pub body_md: String,
}

fn validate_not_blank(value: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        return Err(ValidationError::new("blank"));
    }
    Ok(())
}
