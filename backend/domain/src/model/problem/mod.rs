use super::atcoder_problems::ApiProblem;

pub struct Problem {
    id: String,
    contest_code: String,
    problem_index: String,
    title: String,
}

impl From<ApiProblem> for Problem {
    fn from(value: ApiProblem) -> Self {
        Self {
            id: value.id,
            contest_code: value.contest_id,
            problem_index: value.problem_index,
            title: value.name
        }
    }
}
