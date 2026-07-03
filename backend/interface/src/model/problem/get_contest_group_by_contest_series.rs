use std::{cmp::Reverse, collections::BTreeMap};

use serde::{Deserialize, Serialize};

use crate::model::problem::ProblemResponse;

#[derive(Deserialize)]
pub struct GetContestGroupByContestSeriesRequestParams {
    pub series: String,
    pub q: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContestGroupPageResponse {
    pub items: BTreeMap<Reverse<String>, Vec<ProblemResponse>>,
    pub has_more: bool,
    pub total_contest_count: usize,
}
