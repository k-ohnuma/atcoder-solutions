use std::{cmp::Reverse, collections::BTreeMap, sync::Arc};

use derive_new::new;
use domain::{
    model::problem::{ContestSeries, Problem},
    ports::repository::problem::ProblemRepository,
};

use super::ProblemError;

pub struct ContestGroupCollection(pub BTreeMap<Reverse<String>, Vec<Problem>>);

impl From<Vec<Problem>> for ContestGroupCollection {
    fn from(value: Vec<Problem>) -> Self {
        let mut map = BTreeMap::new();
        for p in value {
            let contest = p.contest_code.to_owned();
            map.entry(Reverse(contest)).or_insert(Vec::new()).push(p);
        }
        Self(map)
    }
}

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
