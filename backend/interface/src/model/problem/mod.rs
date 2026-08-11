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
    pub difficulty_band: Option<String>,
}

impl From<ProblemOutput> for ProblemResponse {
    fn from(value: ProblemOutput) -> Self {
        Self {
            id: value.id.into_string(),
            contest_code: value.contest_code.into_string(),
            problem_index: value.problem_index.into_string(),
            title: value.title.into_string(),
            difficulty: value.difficulty.map(|difficulty| difficulty.value()),
            difficulty_band: value.difficulty_band.map(|band| band.as_str().to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use domain::model::problem::{
        Problem, contest_code::ContestCode, difficulty::Difficulty, id::ProblemId,
        index::ProblemIndex, title::ProblemTitle,
    };

    use super::ProblemResponse;
    use usecase::model::problem::output::ProblemOutput;

    #[test]
    fn response_contains_difficulty_and_derived_band() {
        let problem = Problem {
            id: ProblemId::try_new("abc300_d".to_string()).expect("valid problem ID"),
            contest_code: ContestCode::try_new("abc300".to_string()).expect("valid contest code"),
            problem_index: ProblemIndex::try_new("D".to_string()).expect("valid problem index"),
            title: ProblemTitle::try_new("AABCC".to_string()).expect("valid problem title"),
            difficulty: Some(Difficulty::new(1279)),
        };

        let response = ProblemResponse::from(ProblemOutput::from(problem));
        let json = serde_json::to_value(response).expect("serializable response");

        assert_eq!(json["difficulty"], 1279);
        assert_eq!(json["difficultyBand"], "cyan");
    }
}
