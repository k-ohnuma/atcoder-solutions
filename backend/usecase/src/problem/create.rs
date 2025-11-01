use std::sync::Arc;

use derive_new::new;
use domain::{
    model::problem::Problem,
    ports::{
        external::atcoder_problems::AtcoderProblemsPort, repository::problem::ProblemRepository,
    },
};
use itertools::Itertools;

use crate::model::problem::create::ImportProblemsUsecaseError;

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
