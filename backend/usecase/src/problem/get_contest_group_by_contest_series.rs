use std::sync::Arc;

use derive_new::new;
use domain::{model::problem::ContestSeries, ports::repository::problem::ProblemRepository};

use crate::model::problem::{
    ProblemError, get_contest_group_by_contest_series::ContestGroupCollection,
};

#[derive(new)]
pub struct GetContestGroupByContestSeriesUsecase {
    problem_repository: Arc<dyn ProblemRepository>,
}

impl GetContestGroupByContestSeriesUsecase {
    pub async fn run(&self, series: ContestSeries) -> Result<ContestGroupCollection, ProblemError> {
        let mut pbs = self
            .problem_repository
            .get_problems_by_contest_series(series)
            .await
            .map_err(ProblemError::from)?;

        pbs.sort_by(|a, b| {
            let apid = a.problem_index.to_lowercase();
            let bpid = b.problem_index.to_lowercase();
            if apid == "ex" {
                return std::cmp::Ordering::Greater;
            } else if bpid == "ex" {
                return std::cmp::Ordering::Less;
            }
            apid.cmp(&bpid)
        });

        Ok(pbs.into())
    }
}
