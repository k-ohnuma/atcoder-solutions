use std::sync::Arc;

use derive_new::new;
use domain::{model::problem::ContestSeries, ports::repository::problem::ProblemRepository};

use crate::model::problem::{
    ProblemError,
    get_contest_group_by_contest_series::{ContestGroupCollection, ContestGroupPage},
};

const DEFAULT_CONTEST_LIMIT: usize = 50;
const MAX_CONTEST_LIMIT: usize = 100;

#[derive(new)]
pub struct GetContestGroupByContestSeriesUsecase {
    problem_repository: Arc<dyn ProblemRepository>,
}

impl GetContestGroupByContestSeriesUsecase {
    pub async fn run(
        &self,
        series: ContestSeries,
        query: Option<String>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<ContestGroupPage, ProblemError> {
        let normalized_query = query
            .map(|q| q.trim().to_string())
            .filter(|q| !q.is_empty());

        let (mut pbs, has_more) = if let Some(query) = normalized_query {
            (
                self.problem_repository
                    .search_problems_by_contest_series(series, &query)
                    .await
                    .map_err(ProblemError::from)?,
                false,
            )
        } else {
            let limit = limit
                .unwrap_or(DEFAULT_CONTEST_LIMIT)
                .clamp(1, MAX_CONTEST_LIMIT);
            let offset = offset.unwrap_or(0);
            let mut contest_codes = self
                .problem_repository
                .get_contest_codes_by_series(series, (limit + 1) as i64, offset as i64)
                .await
                .map_err(ProblemError::from)?;
            let has_more = contest_codes.len() > limit;
            contest_codes.truncate(limit);

            (
                self.problem_repository
                    .get_problems_by_contest_codes(&contest_codes)
                    .await
                    .map_err(ProblemError::from)?,
                has_more,
            )
        };

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

        let items = ContestGroupCollection::from(pbs);
        let total_contest_count = items.0.len();

        Ok(ContestGroupPage {
            items,
            has_more,
            total_contest_count,
        })
    }
}
