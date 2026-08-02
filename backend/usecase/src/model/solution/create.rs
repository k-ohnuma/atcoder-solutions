use domain::model::problem::id::ProblemId;
use validator::{Validate, ValidationError};

use crate::error::SolutionError;

#[derive(Validate)]
pub struct CreateSolutionInput {
    #[validate(length(min = 1, max = 120), custom(function = "validate_not_blank"))]
    pub user_id: String,
    pub title: String,
    pub problem_id: ProblemId,
    pub body_md: String,
    pub submit_url: String,
    pub tags: Vec<String>,
}

impl CreateSolutionInput {
    pub fn try_new(
        user_id: String,
        problem_id: String,
        title: String,
        body_md: String,
        submit_url: String,
        tags: Vec<String>,
    ) -> Result<Self, SolutionError> {
        Ok(Self {
            user_id,
            problem_id: ProblemId::try_new(problem_id)?,
            title,
            body_md,
            submit_url,
            tags,
        })
    }
}

fn validate_not_blank(value: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        return Err(ValidationError::new("blank"));
    }
    Ok(())
}
