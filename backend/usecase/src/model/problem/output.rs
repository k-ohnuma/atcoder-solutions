use domain::model::problem::{
    Problem, contest_code::ContestCode, id::ProblemId, index::ProblemIndex, title::ProblemTitle,
};

pub struct ProblemOutput {
    pub id: ProblemId,
    pub contest_code: ContestCode,
    pub problem_index: ProblemIndex,
    pub title: ProblemTitle,
    pub difficulty: Option<i32>,
}

pub struct ContestProblemGroupOutput {
    pub contest_id: ContestCode,
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
