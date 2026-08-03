use domain::model::{problem::id::ProblemId, user::id::UserId};
use validator::Validate;

use crate::error::SolutionError;

#[derive(Validate)]
pub struct CreateSolutionInput {
    pub user_id: UserId,
    pub title: String,
    pub problem_id: ProblemId,
    pub body_md: String,
    pub submit_url: String,
    pub tags: Vec<String>,
}

impl CreateSolutionInput {
    pub fn try_new(
        user_id: UserId,
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
