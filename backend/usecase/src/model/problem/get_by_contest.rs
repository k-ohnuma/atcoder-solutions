use crate::error::ProblemError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetProblemsByContestInput {
    pub contest: String,
}

impl GetProblemsByContestInput {
    pub fn try_new(contest: String) -> Result<Self, ProblemError> {
        let contest = contest.trim();
        if contest.is_empty() {
            return Err(ProblemError::BadRequest(
                "contestCode cannot be empty".to_string(),
            ));
        }
        Ok(Self {
            contest: contest.to_string(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetProblemByIdInput {
    pub problem_id: String,
}

impl GetProblemByIdInput {
    pub fn try_new(problem_id: String) -> Result<Self, ProblemError> {
        let problem_id = problem_id.trim();
        if problem_id.is_empty() {
            return Err(ProblemError::BadRequest(
                "problemId cannot be empty".to_string(),
            ));
        }
        Ok(Self {
            problem_id: problem_id.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{GetProblemByIdInput, GetProblemsByContestInput};
    use crate::error::ProblemError;

    #[test]
    fn contest_input_normalizes_contest_code() {
        let input = GetProblemsByContestInput::try_new(" abc001 ".to_string()).expect("valid");

        assert_eq!(input.contest, "abc001");
    }

    #[test]
    fn contest_input_rejects_blank_contest_code() {
        assert!(matches!(
            GetProblemsByContestInput::try_new("   ".to_string()),
            Err(ProblemError::BadRequest(message)) if message == "contestCode cannot be empty"
        ));
    }

    #[test]
    fn problem_id_input_normalizes_problem_id() {
        let input = GetProblemByIdInput::try_new(" abc001_a ".to_string()).expect("valid");

        assert_eq!(input.problem_id, "abc001_a");
    }

    #[test]
    fn problem_id_input_rejects_blank_problem_id() {
        assert!(matches!(
            GetProblemByIdInput::try_new("   ".to_string()),
            Err(ProblemError::BadRequest(message)) if message == "problemId cannot be empty"
        ));
    }
}
