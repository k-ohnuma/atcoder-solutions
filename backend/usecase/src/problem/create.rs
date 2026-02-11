use std::sync::Arc;

use derive_new::new;
use domain::ports::{
    external::atcoder_problems::AtcoderProblemsPort, repository::problem::ProblemRepository,
};

use crate::model::problem::create::ImportProblemsUsecaseError;

#[derive(new)]
pub struct ImportProblemsUsecase {
    atcoder_problems_port: Arc<dyn AtcoderProblemsPort>,
    problem_repository: Arc<dyn ProblemRepository>,
}

impl ImportProblemsUsecase {
    pub async fn run(&self) -> Result<(), ImportProblemsUsecaseError> {
        let problems = self.atcoder_problems_port.fetch_problems().await?;
        self.problem_repository.create_records(problems).await?;
        Ok(())
    }
}
