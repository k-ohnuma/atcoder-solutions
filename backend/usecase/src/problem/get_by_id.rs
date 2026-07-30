use std::sync::Arc;

use derive_new::new;
use domain::{model::problem::Problem, ports::repository::problem::ProblemRepository};

use crate::model::problem::{ProblemError, get_by_contest::GetProblemByIdInput};

#[derive(new)]
pub struct GetProblemByIdUsecase {
    problem_repository: Arc<dyn ProblemRepository>,
}

impl GetProblemByIdUsecase {
    pub async fn run(&self, input: GetProblemByIdInput) -> Result<Problem, ProblemError> {
        self.problem_repository
            .get_problem_by_id(&input.problem_id)
            .await
            .map_err(ProblemError::from)
    }
}
