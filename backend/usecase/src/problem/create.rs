use std::collections::HashSet;
use std::sync::Arc;

use derive_new::new;
use domain::{
    model::problem::ContestSeries,
    ports::{
        external::atcoder_problems::AtcoderProblemsPort, repository::problem::tx::ProblemTxManager,
    },
};
use tracing::info;

use crate::model::problem::create::ImportProblemsUsecaseError;

const PROBLEM_UPSERT_CHUNK_SIZE: usize = 500;

#[derive(new)]
pub struct ImportProblemsUsecase {
    atcoder_problems_port: Arc<dyn AtcoderProblemsPort>,
    problem_tx_manager: Arc<dyn ProblemTxManager>,
}

impl ImportProblemsUsecase {
    pub async fn run(&self) -> Result<(), ImportProblemsUsecaseError> {
        info!("problem import started");
        let problems = self.atcoder_problems_port.fetch_problems().await?;
        info!(problems = problems.len(), "problem import fetched");

        let contests = problems
            .iter()
            .map(|problem| {
                let series = ContestSeries::try_from(problem.contest_code.as_str())
                    .unwrap_or(ContestSeries::OTHER);
                let series_code: &str = series.into();
                (problem.contest_code.clone(), series_code.to_string())
            })
            .collect::<HashSet<(String, String)>>();

        let contests = contests.into_iter().collect::<Vec<(String, String)>>();
        let mut uow = self.problem_tx_manager.begin().await?;
        info!(contests = contests.len(), "problem import upserting contests");
        uow.problems().upsert_contests_bulk(&contests).await?;
        info!(
            total = problems.len(),
            chunk_size = PROBLEM_UPSERT_CHUNK_SIZE,
            "problem import upserting problems in chunks"
        );

        for (chunk_index, chunk) in problems.chunks(PROBLEM_UPSERT_CHUNK_SIZE).enumerate() {
            uow.problems().upsert_problems_bulk(chunk).await?;
            let processed = ((chunk_index + 1) * PROBLEM_UPSERT_CHUNK_SIZE).min(problems.len());
            info!(
                chunk = chunk_index + 1,
                chunk_size = chunk.len(),
                processed,
                total = problems.len(),
                "problem import chunk completed"
            );
        }
        uow.commit().await?;
        info!("problem import committed");
        Ok(())
    }
}
