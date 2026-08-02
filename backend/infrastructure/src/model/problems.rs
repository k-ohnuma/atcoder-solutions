// 使う想定ではないがdocがわりにDB型明記しておく
use chrono::{DateTime, Utc};
use domain::model::problem::{
    ContestSeries, Problem, contest_code::ContestCode, difficulty::Difficulty, id::ProblemId,
    index::ProblemIndex, title::ProblemTitle,
};

pub struct ProblemRow {
    // abc001_a
    pub id: String,
    // abc001
    pub contest_code: String,
    // a
    pub problem_index: String,
    // title
    pub title: String,
    // atcoder difficulty
    pub difficulty: Option<i32>,
}

impl TryFrom<ProblemRow> for Problem {
    type Error = domain::error::problem::ProblemValidationError;

    fn try_from(value: ProblemRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ProblemId::try_new(value.id)?,
            contest_code: ContestCode::try_new(value.contest_code)?,
            problem_index: ProblemIndex::try_new(value.problem_index)?,
            title: ProblemTitle::try_new(value.title)?,
            difficulty: value.difficulty.map(Difficulty::new),
        })
    }
}

pub struct ContestRow {
    // uuid
    pub id: String,
    // ABC001
    pub code: String,
    // ABC
    pub series_code: ContestSeries,
    // 作成日時
    pub created_at: DateTime<Utc>,
    // 更新日時
    pub update_at: DateTime<Utc>,
}

pub struct ContestSeriesRow {
    // uuid
    pub id: String,
    // ABC, ARC, ...
    pub code: ContestSeries,
    // 作成日時
    pub created_at: DateTime<Utc>,
}
