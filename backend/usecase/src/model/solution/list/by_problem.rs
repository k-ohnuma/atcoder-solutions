use domain::read_model::solution::SolutionListSort;

use crate::error::SolutionError;

use super::{normalize_required_string, validate_limit};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSolutionsByProblemIdInput {
    pub problem_id: String,
    pub sort: SolutionListSort,
    pub limit: Option<i32>,
}

impl GetSolutionsByProblemIdInput {
    pub fn try_new(
        problem_id: String,
        sort_by: Option<String>,
        limit: Option<i32>,
    ) -> Result<Self, SolutionError> {
        let problem_id = normalize_required_string(problem_id, "problem_id cannot be empty")?;
        Ok(Self {
            problem_id,
            sort: parse_solution_list_sort(sort_by)?,
            limit: validate_limit(limit)?,
        })
    }
}

pub(super) fn parse_solution_list_sort(
    sort_by: Option<String>,
) -> Result<SolutionListSort, SolutionError> {
    match sort_by.as_deref() {
        None | Some("latest") => Ok(SolutionListSort::Latest),
        Some("votes") => Ok(SolutionListSort::Votes),
        Some(_) => Err(SolutionError::BadRequest(
            "sort_by must be one of: latest, votes".to_string(),
        )),
    }
}
