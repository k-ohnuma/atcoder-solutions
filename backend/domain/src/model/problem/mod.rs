use strum::Display;
use thiserror::Error;

pub mod contest_code;
pub mod difficulty;
pub mod id;
pub mod index;
pub mod title;

use contest_code::ContestCode;
use difficulty::Difficulty;
use id::ProblemId;
use index::ProblemIndex;
use title::ProblemTitle;

#[derive(Clone, Debug)]
pub struct Problem {
    pub id: ProblemId,
    pub contest_code: ContestCode,
    pub problem_index: ProblemIndex,
    pub title: ProblemTitle,
    pub difficulty: Option<Difficulty>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Display)]
pub enum ContestSeries {
    ABC,
    ARC,
    AGC,
    AHC,
    AWC,
    OTHER,
}

impl ContestSeries {
    pub fn from_contest_code(value: &str) -> Self {
        let lower = value.to_ascii_lowercase();
        if lower.starts_with("abc") {
            Self::ABC
        } else if lower.starts_with("arc") {
            Self::ARC
        } else if lower.starts_with("agc") {
            Self::AGC
        } else if lower.starts_with("ahc") {
            Self::AHC
        } else if lower.starts_with("awc") {
            Self::AWC
        } else {
            Self::OTHER
        }
    }
}

impl TryFrom<&str> for ContestSeries {
    type Error = ContestSeriesParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase().as_str() {
            "ABC" => Ok(Self::ABC),
            "ARC" => Ok(Self::ARC),
            "AGC" => Ok(Self::AGC),
            "AHC" => Ok(Self::AHC),
            "AWC" => Ok(Self::AWC),
            "OTHER" => Ok(Self::OTHER),
            _ => Err(ContestSeriesParseError::Invalid {
                input: value.into(),
            }),
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
            ContestSeries::AWC => "AWC".into(),
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
            ContestSeries::AWC => "AWC",
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
            ContestSeriesParseError::Invalid { input } => input.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ContestSeries;

    #[test]
    fn parses_series_code_by_exact_match() {
        assert_eq!(
            ContestSeries::try_from("abc").expect("valid series code"),
            ContestSeries::ABC
        );
        assert!(ContestSeries::try_from("abc001").is_err());
    }

    #[test]
    fn classifies_contest_code_by_prefix() {
        assert_eq!(
            ContestSeries::from_contest_code("abc001"),
            ContestSeries::ABC
        );
        assert_eq!(
            ContestSeries::from_contest_code("tessoku_book"),
            ContestSeries::OTHER
        );
    }
}
