use domain::model::solution::Solution;
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Validate)]
pub struct CreateSolutionInput {
    #[validate(length(min = 1, max = 120), custom(function = "validate_not_blank"))]
    pub user_id: String,
    #[validate(length(min = 1, max = 120), custom(function = "validate_not_blank"))]
    pub title: String,
    #[validate(length(min = 1), custom(function = "validate_not_blank"))]
    pub problem_id: String,
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

pub fn from_create_solution_input_for_solution(
    uuid: Uuid,
    input: &CreateSolutionInput,
) -> Solution {
    Solution {
        id: uuid,
        title: input.title.to_owned(),
        user_id: input.user_id.to_owned(),
        problem_id: input.problem_id.to_owned(),
        body_md: input.body_md.to_owned(),
        submit_url: input.submit_url.to_owned(),
    }
}
