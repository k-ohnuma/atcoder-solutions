use super::{SolutionError, SolutionListSort};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetLatestSolutionsInput {
    pub limit: Option<i32>,
}

impl GetLatestSolutionsInput {
    pub fn try_new(sort_by: Option<String>, limit: Option<i32>) -> Result<Self, SolutionError> {
        if matches!(sort_by.as_deref(), Some(v) if v != "latest") {
            return Err(SolutionError::BadRequest(
                "sort_by must be latest".to_string(),
            ));
        }
        Ok(Self {
            limit: validate_limit(limit)?,
        })
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSolutionsByUserNameInput {
    pub user_name: String,
    pub sort: SolutionListSort,
}

impl GetSolutionsByUserNameInput {
    pub fn try_new(user_name: String, sort_by: Option<String>) -> Result<Self, SolutionError> {
        let user_name = normalize_required_string(user_name, "user_name cannot be empty")?;
        Ok(Self {
            user_name,
            sort: parse_solution_list_sort(sort_by)?,
        })
    }
}

fn validate_limit(limit: Option<i32>) -> Result<Option<i32>, SolutionError> {
    match limit {
        Some(value) if value <= 0 => Err(SolutionError::BadRequest(
            "limit must be greater than 0".to_string(),
        )),
        _ => Ok(limit),
    }
}

fn parse_solution_list_sort(sort_by: Option<String>) -> Result<SolutionListSort, SolutionError> {
    match sort_by.as_deref() {
        None | Some("latest") => Ok(SolutionListSort::Latest),
        Some("votes") => Ok(SolutionListSort::Votes),
        Some(_) => Err(SolutionError::BadRequest(
            "sort_by must be one of: latest, votes".to_string(),
        )),
    }
}

fn normalize_required_string(value: String, error_message: &str) -> Result<String, SolutionError> {
    let normalized = value.trim();
    if normalized.is_empty() {
        return Err(SolutionError::BadRequest(error_message.to_string()));
    }
    Ok(normalized.to_string())
}

#[cfg(test)]
mod tests {
    use super::{
        GetLatestSolutionsInput, GetSolutionsByProblemIdInput, GetSolutionsByUserNameInput,
    };
    use crate::model::solution::{SolutionError, SolutionListSort};

    #[test]
    fn latest_input_accepts_empty_or_latest_sort() {
        assert_eq!(
            GetLatestSolutionsInput::try_new(None, Some(50))
                .expect("valid")
                .limit,
            Some(50)
        );
        assert_eq!(
            GetLatestSolutionsInput::try_new(Some("latest".to_string()), None)
                .expect("valid")
                .limit,
            None
        );
    }

    #[test]
    fn latest_input_rejects_other_sort() {
        assert!(matches!(
            GetLatestSolutionsInput::try_new(Some("votes".to_string()), None),
            Err(SolutionError::BadRequest(message)) if message == "sort_by must be latest"
        ));
    }

    #[test]
    fn list_input_rejects_non_positive_limit() {
        assert!(matches!(
            GetSolutionsByProblemIdInput::try_new("abc100_a".to_string(), None, Some(0)),
            Err(SolutionError::BadRequest(message)) if message == "limit must be greater than 0"
        ));
    }

    #[test]
    fn problem_id_input_normalizes_values() {
        let input = GetSolutionsByProblemIdInput::try_new(" abc100_a ".to_string(), None, Some(20))
            .expect("valid");

        assert_eq!(input.problem_id, "abc100_a");
        assert!(matches!(input.sort, SolutionListSort::Latest));
        assert_eq!(input.limit, Some(20));
    }

    #[test]
    fn problem_id_input_accepts_votes_sort() {
        let input = GetSolutionsByProblemIdInput::try_new(
            "abc100_a".to_string(),
            Some("votes".to_string()),
            None,
        )
        .expect("valid");

        assert!(matches!(input.sort, SolutionListSort::Votes));
    }

    #[test]
    fn problem_id_input_rejects_blank_problem_id() {
        assert!(matches!(
            GetSolutionsByProblemIdInput::try_new("   ".to_string(), None, None),
            Err(SolutionError::BadRequest(message)) if message == "problem_id cannot be empty"
        ));
    }

    #[test]
    fn user_name_input_rejects_invalid_sort() {
        assert!(matches!(
            GetSolutionsByUserNameInput::try_new(
                "alice".to_string(),
                Some("unknown".to_string())
            ),
            Err(SolutionError::BadRequest(message)) if message == "sort_by must be one of: latest, votes"
        ));
    }

    #[test]
    fn user_name_input_normalizes_values() {
        let input =
            GetSolutionsByUserNameInput::try_new(" alice ".to_string(), Some("votes".to_string()))
                .expect("valid");

        assert_eq!(input.user_name, "alice");
        assert!(matches!(input.sort, SolutionListSort::Votes));
    }
}
