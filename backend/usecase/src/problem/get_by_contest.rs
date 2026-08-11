use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::problem::ProblemRepository;

use crate::{
    error::ProblemError,
    model::problem::{get_by_contest::GetProblemsByContestInput, output::ProblemOutput},
};

#[derive(new)]
pub struct GetProblemsByContestUsecase {
    problem_repository: Arc<dyn ProblemRepository>,
}

impl GetProblemsByContestUsecase {
    pub async fn run(
        &self,
        input: GetProblemsByContestInput,
    ) -> Result<Vec<ProblemOutput>, ProblemError> {
        let mut pbs = self
            .problem_repository
            .get_problems_by_contest(&input.contest)
            .await
            .map_err(ProblemError::from)?;
        pbs.sort_by(|a, b| a.problem_index.cmp_for_display(&b.problem_index));

        Ok(pbs.into_iter().map(ProblemOutput::from).collect())
    }
}
