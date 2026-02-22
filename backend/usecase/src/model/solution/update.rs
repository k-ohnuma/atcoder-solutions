use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Validate)]
pub struct UpdateSolutionInput {
    #[validate(length(min = 1, max = 120), custom(function = "validate_not_blank"))]
    pub user_id: String,
    pub solution_id: Uuid,
    #[validate(length(min = 1, max = 120), custom(function = "validate_not_blank"))]
    pub title: String,
    #[validate(length(min = 1, max = 20000), custom(function = "validate_not_blank"))]
    pub body_md: String,
    #[validate(length(max = 500), custom(function = "validate_http_url"))]
    pub submit_url: String,
    #[validate(length(max = 6))]
    pub tags: Vec<String>,
}

fn validate_not_blank(value: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        return Err(ValidationError::new("blank"));
    }
    Ok(())
}

fn validate_http_url(value: &str) -> Result<(), ValidationError> {
    let normalized = value.trim();
    if normalized.is_empty() {
        return Ok(());
    }
    if !(normalized.starts_with("https://") || normalized.starts_with("http://")) {
        return Err(ValidationError::new("scheme"));
    }
    Ok(())
}
