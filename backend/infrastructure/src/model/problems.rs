// 使う想定ではないがdocがわりにDB型明記しておく
use chrono::{DateTime, Utc};
use domain::model::problem::ContestSeries;

pub struct ProblemRow {
    // abc001_a
    pub id: String,
    // abc001
    pub contest_code: String,
    // a
    pub problem_index: String,
    // title
    pub title: String,
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
