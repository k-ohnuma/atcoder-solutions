use std::{cmp::Reverse, collections::BTreeMap, sync::Arc};

use derive_new::new;
use domain::{model::problem::Problem, ports::repository::problem::ProblemRepository};

use crate::{
    error::ProblemError,
    model::problem::{
        get_contest_group_by_contest_series::GetContestGroupByContestSeriesInput,
        output::{ContestGroupPageOutput, ContestProblemGroupOutput, ProblemOutput},
    },
};

const DEFAULT_CONTEST_LIMIT: usize = 50;
const MAX_CONTEST_LIMIT: usize = 100;

#[derive(new)]
pub struct GetContestGroupByContestSeriesUsecase {
    problem_repository: Arc<dyn ProblemRepository>,
}

impl GetContestGroupByContestSeriesUsecase {
    pub async fn run(
        &self,
        input: GetContestGroupByContestSeriesInput,
    ) -> Result<ContestGroupPageOutput, ProblemError> {
        let (mut pbs, has_more) = if let Some(query) = input.query {
            (
                self.problem_repository
                    .search_problems_by_contest_series(input.series, &query)
                    .await
                    .map_err(ProblemError::from)?,
                false,
            )
        } else {
            let limit = input
                .limit
                .unwrap_or(DEFAULT_CONTEST_LIMIT)
                .clamp(1, MAX_CONTEST_LIMIT);
            let offset = input.offset.unwrap_or(0);
            let mut contest_codes = self
                .problem_repository
                .get_contest_codes_by_series(input.series, (limit + 1) as i64, offset as i64)
                .await
                .map_err(ProblemError::from)?;
            let has_more = contest_codes.len() > limit;
            contest_codes.truncate(limit);

            (
                self.problem_repository
                    .get_problems_by_contest_codes(&contest_codes)
                    .await
                    .map_err(ProblemError::from)?,
                has_more,
            )
        };

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

        let groups = group_problems_by_contest(pbs);
        let total_contest_count = groups.len();

        Ok(ContestGroupPageOutput {
            groups,
            has_more,
            total_contest_count,
        })
    }
}

fn group_problems_by_contest(problems: Vec<Problem>) -> Vec<ContestProblemGroupOutput> {
    let mut groups = BTreeMap::new();
    for problem in problems {
        groups
            .entry(Reverse(problem.contest_code.clone()))
            .or_insert_with(Vec::new)
            .push(problem);
    }

    groups
        .into_iter()
        .map(
            |(Reverse(contest_id), problems)| ContestProblemGroupOutput {
                contest_id,
                problems: problems.into_iter().map(ProblemOutput::from).collect(),
            },
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use domain::model::problem::Problem;

    use super::group_problems_by_contest;

    #[test]
    fn groups_problems_by_contest_in_descending_order() {
        let groups = group_problems_by_contest(vec![
            problem("abc299_a", "abc299", "A"),
            problem("abc300_b", "abc300", "B"),
            problem("abc300_a", "abc300", "A"),
        ]);

        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].contest_id, "abc300");
        assert_eq!(groups[0].problems.len(), 2);
        assert_eq!(groups[0].problems[0].id.as_str(), "abc300_b");
        assert_eq!(groups[1].contest_id, "abc299");
        assert_eq!(groups[1].problems[0].id.as_str(), "abc299_a");
    }

    fn problem(id: &str, contest_code: &str, problem_index: &str) -> Problem {
        Problem {
            id: domain::model::problem::id::ProblemId::try_new(id.to_string())
                .expect("valid problem ID"),
            contest_code: contest_code.to_string(),
            problem_index: problem_index.to_string(),
            title: id.to_string(),
            difficulty: None,
        }
    }
}
