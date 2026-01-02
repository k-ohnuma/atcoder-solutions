use strum::Display;
use thiserror::Error;

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

impl TryFrom<&str> for ContestSeries {
    type Error = ContestSeriesParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lower = value.to_ascii_lowercase();
        if lower.starts_with("abc") {
            Ok(ContestSeries::ABC)
        } else if lower.starts_with("arc") {
            Ok(ContestSeries::ARC)
        } else if lower.starts_with("agc") {
            Ok(ContestSeries::AGC)
        } else if lower.starts_with("ahc") {
            Ok(ContestSeries::AHC)
        } else {
            Err(ContestSeriesParseError::Invalid {
                input: value.into(),
            })
        }
    }
}
impl TryFrom<String> for ContestSeries {
    type Error = ContestSeriesParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        ContestSeries::try_from(value.as_str())
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

#[derive(Debug, Error)]
pub enum ContestSeriesParseError {
    #[error("invalid contest series: {input}")]
    Invalid { input: String },
}

impl ContestSeriesParseError {
    pub fn msg(&self) -> String {
        match self {
            ContestSeriesParseError::Invalid { input } => input.to_string()
        }
    }
}
