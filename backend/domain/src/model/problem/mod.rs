use strum::Display;

use super::atcoder_problems::ApiProblem;

#[derive(Clone, Debug)]
pub struct Problem {
    pub id: String,
    pub contest_code: String,
    pub problem_index: String,
    pub title: String,
}

impl From<ApiProblem> for Problem {
    fn from(value: ApiProblem) -> Self {
        Self {
            id: value.id,
            contest_code: value.contest_id,
            problem_index: value.problem_index,
            title: value.name,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Display)]
pub enum ContestSeries {
    ABC,
    ARC,
    AHC,
    AGC,
    OTHER,
}

impl From<String> for ContestSeries {
    fn from(value: String) -> Self {
        let lower = value.to_ascii_lowercase();
        if lower.starts_with("abc") {
            ContestSeries::ABC
        } else if lower.starts_with("arc") {
            ContestSeries::ARC
        } else if lower.starts_with("agc") {
            ContestSeries::AGC
        } else if lower.starts_with("ahc") {
            ContestSeries::AHC
        } else {
            ContestSeries::OTHER
        }
    }
}
impl From<&str> for ContestSeries {
    fn from(value: &str) -> Self {
        let lower = value.to_ascii_lowercase();
        if lower.starts_with("abc") {
            ContestSeries::ABC
        } else if lower.starts_with("arc") {
            ContestSeries::ARC
        } else if lower.starts_with("agc") {
            ContestSeries::AGC
        } else if lower.starts_with("ahc") {
            ContestSeries::AHC
        } else {
            ContestSeries::OTHER
        }
    }
}

impl From<ContestSeries> for String {
    fn from(value: ContestSeries) -> Self {
        match value {
            ContestSeries::ABC => "ABC".into(),
            ContestSeries::ARC => "ARC".into(),
            ContestSeries::AGC => "AGC".into(),
            ContestSeries::AHC => "AHC".into(),
            _ => "OTHER".into(),
        }
    }
}

impl From<ContestSeries> for &str {
    fn from(value: ContestSeries) -> Self {
        match value {
            ContestSeries::ABC => "ABC",
            ContestSeries::ARC => "ARC",
            ContestSeries::AGC => "AGC",
            ContestSeries::AHC => "AHC",
            _ => "OTHER",
        }
    }
}
