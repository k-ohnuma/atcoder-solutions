use uuid::Uuid;
use validator::{Validate, ValidationError};

use domain::model::user::id::UserId;

#[derive(Validate)]
pub struct UpdateCommentInput {
    pub user_id: UserId,
    pub comment_id: Uuid,
    #[validate(length(min = 1, max = 2000), custom(function = "validate_not_blank"))]
    pub body_md: String,
}

fn validate_not_blank(value: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        return Err(ValidationError::new("blank"));
    }
    Ok(())
}
