use serde::Serialize;
use usecase::model::problem::output::ProblemOutput;

pub mod get_contest_group_by_contest_series;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProblemResponse {
    pub id: String,
    pub contest_code: String,
    pub problem_index: String,
    pub title: String,
    pub difficulty: Option<i32>,
}

impl From<ProblemOutput> for ProblemResponse {
    fn from(value: ProblemOutput) -> Self {
        Self {
            id: value.id.into_string(),
            contest_code: value.contest_code,
            problem_index: value.problem_index,
            title: value.title,
            difficulty: value.difficulty,
        }
    }
}
