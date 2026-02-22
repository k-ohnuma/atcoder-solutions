use std::sync::Arc;

use derive_new::new;
use domain::{model::problem::Problem, ports::repository::problem::ProblemRepository};

use crate::model::problem::ProblemError;

#[derive(new)]
pub struct GetProblemsByContestUsecase {
    problem_repository: Arc<dyn ProblemRepository>,
}

impl GetProblemsByContestUsecase {
    pub async fn run(&self, contest: &str) -> Result<Vec<Problem>, ProblemError> {
        if contest.trim().is_empty() {
            return Err(ProblemError::BadRequest(
                "contest cannot be empty".to_string(),
            ));
        }

        let mut pbs = self
            .problem_repository
            .get_problems_by_contest(contest)
            .await
            .map_err(ProblemError::from)?;
        pbs.sort_by(|a, b| {
            let apid = a.problem_index.to_lowercase();
            let bpid = b.problem_index.to_lowercase();
            if apid == "ex" {
                return std::cmp::Ordering::Greater;
            } else if bpid == "ex" {
                return std::cmp::Ordering::Less;
            }
            apid.cmp(&bpid)
        });

        Ok(pbs)
    }
}
