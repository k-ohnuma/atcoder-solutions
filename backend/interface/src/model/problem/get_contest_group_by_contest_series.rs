use serde::{Deserialize, Serialize};
use usecase::model::problem::output::{ContestGroupPageOutput, ContestProblemGroupOutput};

use crate::model::problem::ProblemResponse;

#[derive(Deserialize)]
pub struct GetContestGroupByContestSeriesRequestParams {
    pub q: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContestProblemGroupResponse {
    pub contest_id: String,
    pub problems: Vec<ProblemResponse>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContestGroupPageResponse {
    pub groups: Vec<ContestProblemGroupResponse>,
    pub has_more: bool,
    pub total_contest_count: usize,
}

impl From<ContestProblemGroupOutput> for ContestProblemGroupResponse {
    fn from(value: ContestProblemGroupOutput) -> Self {
        Self {
            contest_id: value.contest_id.into_string(),
            problems: value
                .problems
                .into_iter()
                .map(ProblemResponse::from)
                .collect(),
        }
    }
}

impl From<ContestGroupPageOutput> for ContestGroupPageResponse {
    fn from(value: ContestGroupPageOutput) -> Self {
        Self {
            groups: value
                .groups
                .into_iter()
                .map(ContestProblemGroupResponse::from)
                .collect(),
            has_more: value.has_more,
            total_contest_count: value.total_contest_count,
        }
    }
}
