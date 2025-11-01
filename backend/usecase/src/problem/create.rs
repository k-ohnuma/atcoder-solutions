use std::sync::Arc;

use derive_new::new;
use domain::{
    model::problem::Problem,
    ports::{
        external::atcoder_problems::AtcoderProblemsPort, repository::problem::ProblemRepository,
    },
};
use itertools::Itertools;
use shared::error::{external::ExternalError, repository::RepositoryError};

#[derive(thiserror::Error, Debug)]
pub enum ImportProblemsUsecaseError {
    #[error(transparent)]
    Fetch(#[from] ExternalError),
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

#[derive(new)]
pub struct ImportProblemsUsecase {
    atcoder_problems_port: Arc<dyn AtcoderProblemsPort>,
    problem_repository: Arc<dyn ProblemRepository>,
}

impl ImportProblemsUsecase {
    pub async fn run(&self) -> Result<(), ImportProblemsUsecaseError> {
        let api_items = self.atcoder_problems_port.fetch_problems().await?;
        let problems = api_items.into_iter().map(Problem::from).collect_vec();
        self.problem_repository.create_records(problems).await?;
        Ok(())
    }
}
