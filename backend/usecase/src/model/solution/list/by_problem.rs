use domain::{model::problem::id::ProblemId, read_model::solution::SolutionListSort};

use crate::error::SolutionError;

use super::validate_limit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSolutionsByProblemIdInput {
    pub problem_id: ProblemId,
    pub sort: SolutionListSort,
    pub limit: Option<i32>,
}

impl GetSolutionsByProblemIdInput {
    pub fn try_new(
        problem_id: String,
        sort_by: Option<String>,
        limit: Option<i32>,
    ) -> Result<Self, SolutionError> {
        let problem_id = ProblemId::try_new(problem_id)?;
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
