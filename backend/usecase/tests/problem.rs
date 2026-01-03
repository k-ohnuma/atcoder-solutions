use std::sync::{Arc, Mutex};

use anyhow::Result;
use async_trait::async_trait;
use domain::{
    model::{
        atcoder_problems::ApiProblem,
        problem::{ContestSeries, Problem},
    },
    ports::{
        external::atcoder_problems::AtcoderProblemsPort, repository::problem::ProblemRepository,
    },
};
use shared::error::{external::ExternalError, repository::RepositoryError};
use usecase::problem::create::ImportProblemsUsecase;

struct DummyAtcoderProblemsPort {
    item: Vec<ApiProblem>,
}

#[async_trait]
impl AtcoderProblemsPort for DummyAtcoderProblemsPort {
    async fn fetch_problems(&self) -> Result<Vec<ApiProblem>, ExternalError> {
        Ok(self.item.to_owned())
    }
}

struct DummyProblemsRepository {
    calls: Mutex<Vec<Problem>>,
}

#[async_trait]
impl ProblemRepository for DummyProblemsRepository {
    async fn create_records(&self, problems: Vec<Problem>) -> Result<(), RepositoryError> {
        self.calls.lock().unwrap().extend(problems);
        Ok(())
    }
    async fn get_problems_by_contest_series(
        &self,
        _series: ContestSeries,
    ) -> Result<Vec<Problem>, RepositoryError> {
        Ok(vec![])
    }
}

#[tokio::test]
async fn usecase_calls_repo_with_converted_problems() -> Result<()> {
    let port = Arc::new(DummyAtcoderProblemsPort {
        item: vec![
            ApiProblem {
                id: "abc234_a".into(),
                contest_id: "abc234".into(),
                problem_index: "A".into(),
                name: "A - Example".into(),
            },
            ApiProblem {
                id: "abc234_b".into(),
                contest_id: "abc234".into(),
                problem_index: "B".into(),
                name: "B - Example".into(),
            },
        ],
    });
    let repo = Arc::new(DummyProblemsRepository {
        calls: Mutex::new(vec![]),
    });

    let uc = ImportProblemsUsecase::new(port, repo.clone());
    uc.run().await.unwrap();

    let calls = repo.calls.lock().unwrap();
    assert_eq!(calls.len(), 2);
    let sent = &calls[0];
    assert_eq!(sent.id, "abc234_a");
    assert_eq!(sent.title, "A - Example");

    Ok(())
}
