mod by_problem;
mod by_user;
mod latest;

use crate::error::SolutionError;

pub use by_problem::GetSolutionsByProblemIdInput;
pub use by_user::GetSolutionsByUserNameInput;
pub use latest::GetLatestSolutionsInput;

fn validate_limit(limit: Option<i32>) -> Result<Option<i32>, SolutionError> {
    match limit {
        Some(value) if value <= 0 => Err(SolutionError::BadRequest(
            "limit must be greater than 0".to_string(),
        )),
        _ => Ok(limit),
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
    use domain::read_model::solution::SolutionListSort;

    use super::{
        GetLatestSolutionsInput, GetSolutionsByProblemIdInput, GetSolutionsByUserNameInput,
    };
    use crate::error::SolutionError;

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
