use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::problem::ProblemRepository;

use crate::{
    error::ProblemError,
    model::problem::{get_by_contest::GetProblemByIdInput, output::ProblemOutput},
};

#[derive(new)]
pub struct GetProblemByIdUsecase {
    problem_repository: Arc<dyn ProblemRepository>,
}

impl GetProblemByIdUsecase {
    pub async fn run(&self, input: GetProblemByIdInput) -> Result<ProblemOutput, ProblemError> {
        self.problem_repository
            .get_problem_by_id(&input.problem_id)
            .await
            .map_err(ProblemError::from)
            .map(ProblemOutput::from)
    }
}
