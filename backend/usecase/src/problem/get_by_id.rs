use std::sync::Arc;

use derive_new::new;
use domain::{model::problem::Problem, ports::repository::problem::ProblemRepository};

use crate::model::problem::ProblemError;

#[derive(new)]
pub struct GetProblemByIdUsecase {
    problem_repository: Arc<dyn ProblemRepository>,
}

impl GetProblemByIdUsecase {
    pub async fn run(&self, problem_id: &str) -> Result<Problem, ProblemError> {
        if problem_id.trim().is_empty() {
            return Err(ProblemError::BadRequest(
                "problemId cannot be empty".to_string(),
            ));
        }

        self.problem_repository
            .get_problem_by_id(problem_id)
            .await
            .map_err(ProblemError::from)
    }
}
