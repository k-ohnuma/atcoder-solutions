use domain::model::problem::Problem;
use serde::Serialize;

pub mod get_problems_by_contest_series;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProblemResponse {
    pub id: String,
    pub contest_code: String,
    pub problem_index: String,
    pub title: String,
}

impl From<Problem> for ProblemResponse {
    fn from(value: Problem) -> Self {
        Self {
            id: value.id,
            contest_code: value.contest_code,
            problem_index: value.problem_index,
            title: value.title,
        }
    }
}
