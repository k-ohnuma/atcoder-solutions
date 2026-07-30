use std::{cmp::Reverse, collections::BTreeMap};

use domain::model::problem::{ContestSeries, Problem};

use crate::error::ProblemError;

const MAX_CONTEST_GROUP_QUERY_LENGTH: usize = 100;
const MAX_CONTEST_GROUP_OFFSET: usize = 5_000;

#[derive(Clone, PartialEq, Eq)]
pub struct GetContestGroupByContestSeriesInput {
    pub series: ContestSeries,
    pub query: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl GetContestGroupByContestSeriesInput {
    pub fn try_new(
        series: String,
        query: Option<String>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Self, ProblemError> {
        let series =
            ContestSeries::try_from(series).map_err(|e| ProblemError::BadRequest(e.msg()))?;
        let query = query
            .map(|q| q.trim().to_string())
            .filter(|q| !q.is_empty());

        if query
            .as_ref()
            .is_some_and(|q| q.chars().count() > MAX_CONTEST_GROUP_QUERY_LENGTH)
        {
            return Err(ProblemError::BadRequest(format!(
                "q must be at most {MAX_CONTEST_GROUP_QUERY_LENGTH} characters"
            )));
        }

        if offset.is_some_and(|offset| offset > MAX_CONTEST_GROUP_OFFSET) {
            return Err(ProblemError::BadRequest(format!(
                "offset must be at most {MAX_CONTEST_GROUP_OFFSET}"
            )));
        }

        Ok(Self {
            series,
            query,
            limit,
            offset,
        })
    }
}

pub struct ContestGroupCollection(pub BTreeMap<Reverse<String>, Vec<Problem>>);

pub struct ContestGroupPage {
    pub items: ContestGroupCollection,
    pub has_more: bool,
    pub total_contest_count: usize,
}

impl From<Vec<Problem>> for ContestGroupCollection {
    fn from(value: Vec<Problem>) -> Self {
        let mut map = BTreeMap::new();
        for p in value {
            let contest = p.contest_code.to_owned();
            map.entry(Reverse(contest)).or_insert(Vec::new()).push(p);
        }
        Self(map)
    }
}

#[cfg(test)]
mod tests {
    use super::GetContestGroupByContestSeriesInput;
    use crate::error::ProblemError;

    #[test]
    fn contest_group_input_normalizes_query() {
        let input = GetContestGroupByContestSeriesInput::try_new(
            "abc".to_string(),
            Some(" abc300 ".to_string()),
            Some(50),
            Some(10),
        )
        .expect("valid");

        assert_eq!(input.query.as_deref(), Some("abc300"));
        assert_eq!(input.limit, Some(50));
        assert_eq!(input.offset, Some(10));
    }

    #[test]
    fn contest_group_input_treats_blank_query_as_empty() {
        let input = GetContestGroupByContestSeriesInput::try_new(
            "abc".to_string(),
            Some("   ".to_string()),
            None,
            None,
        )
        .expect("valid");

        assert_eq!(input.query, None);
    }

    #[test]
    fn contest_group_input_rejects_too_long_query() {
        let query = "a".repeat(101);

        assert!(matches!(
            GetContestGroupByContestSeriesInput::try_new("abc".to_string(), Some(query), None, None),
            Err(ProblemError::BadRequest(message)) if message == "q must be at most 100 characters"
        ));
    }

    #[test]
    fn contest_group_input_rejects_too_large_offset() {
        assert!(matches!(
            GetContestGroupByContestSeriesInput::try_new("abc".to_string(), None, None, Some(5_001)),
            Err(ProblemError::BadRequest(message)) if message == "offset must be at most 5000"
        ));
    }

    #[test]
    fn contest_group_input_rejects_invalid_series() {
        assert!(matches!(
            GetContestGroupByContestSeriesInput::try_new("invalid".to_string(), None, None, None),
            Err(ProblemError::BadRequest(message)) if message == "invalid"
        ));
    }
}
