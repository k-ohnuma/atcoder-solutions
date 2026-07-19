use serde::{Deserialize, Serialize};

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
