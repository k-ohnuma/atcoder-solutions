use domain::model::problem::{Problem, id::ProblemId};

pub struct ProblemOutput {
    pub id: ProblemId,
    pub contest_code: String,
    pub problem_index: String,
    pub title: String,
    pub difficulty: Option<i32>,
}

pub struct ContestProblemGroupOutput {
    pub contest_id: String,
    pub problems: Vec<ProblemOutput>,
}

pub struct ContestGroupPageOutput {
    pub groups: Vec<ContestProblemGroupOutput>,
    pub has_more: bool,
    pub total_contest_count: usize,
}

impl From<Problem> for ProblemOutput {
    fn from(value: Problem) -> Self {
        Self {
            id: value.id,
            contest_code: value.contest_code,
            problem_index: value.problem_index,
            title: value.title,
            difficulty: value.difficulty,
        }
    }
}
