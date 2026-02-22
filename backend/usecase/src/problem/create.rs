use std::sync::Arc;
use std::collections::HashSet;

use derive_new::new;
use domain::{
    model::problem::ContestSeries,
    ports::{
        external::atcoder_problems::AtcoderProblemsPort, repository::problem::tx::ProblemTxManager,
    },
};

use crate::model::problem::create::ImportProblemsUsecaseError;

#[derive(new)]
pub struct ImportProblemsUsecase {
    atcoder_problems_port: Arc<dyn AtcoderProblemsPort>,
    problem_tx_manager: Arc<dyn ProblemTxManager>,
}

impl ImportProblemsUsecase {
    pub async fn run(&self) -> Result<(), ImportProblemsUsecaseError> {
        let problems = self.atcoder_problems_port.fetch_problems().await?;

        let contests = problems
            .iter()
            .map(|problem| {
                let series = ContestSeries::try_from(problem.contest_code.as_str())
                    .unwrap_or(ContestSeries::OTHER);
                let series_code: &str = series.into();
                (problem.contest_code.clone(), series_code.to_string())
            })
            .collect::<HashSet<(String, String)>>();

        let mut uow = self.problem_tx_manager.begin().await?;
        for (contest_code, series_code) in contests {
            uow.problems()
                .upsert_contest(&contest_code, &series_code)
                .await?;
        }
        for problem in problems {
            uow.problems()
                .upsert_problem(
                    &problem.id,
                    &problem.contest_code,
                    &problem.problem_index,
                    &problem.title,
                )
                .await?;
        }
        uow.commit().await?;
        Ok(())
    }
}
