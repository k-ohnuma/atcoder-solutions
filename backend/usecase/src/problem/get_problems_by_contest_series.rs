use std::sync::Arc;

use derive_new::new;
use domain::{
    model::problem::{ContestSeries, Problem},
    ports::repository::problem::ProblemRepository,
};

use super::ProblemError;

#[derive(new)]
pub struct GetProblemsByContestSeriesUsecase {
    problem_repository: Arc<dyn ProblemRepository>,
}

impl GetProblemsByContestSeriesUsecase {
    pub async fn run(&self, series: ContestSeries) -> Result<Vec<Problem>, ProblemError> {
        let pbs = self
            .problem_repository
            .get_problems_by_contest_series(series)
            .await
            .map_err(ProblemError::from)?;
        Ok(pbs)
    }
}
