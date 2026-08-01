use validator::{Validate, ValidationError};

#[derive(Validate)]
pub struct CreateSolutionInput {
    #[validate(length(min = 1, max = 120), custom(function = "validate_not_blank"))]
    pub user_id: String,
    pub title: String,
    #[validate(length(min = 1), custom(function = "validate_not_blank"))]
    pub problem_id: String,
    pub body_md: String,
    pub submit_url: String,
    pub tags: Vec<String>,
}

fn validate_not_blank(value: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        return Err(ValidationError::new("blank"));
    }
    Ok(())
}
