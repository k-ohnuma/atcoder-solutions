use std::sync::{Arc, Mutex};

use anyhow::Result;
use async_trait::async_trait;
use domain::{
    error::{external::ExternalError, repository::RepositoryError},
    model::problem::{ContestSeries, Problem},
    ports::{
        external::atcoder_problems::AtcoderProblemsPort, repository::problem::ProblemRepository,
    },
};
use usecase::problem::create::ImportProblemsUsecase;

struct DummyAtcoderProblemsPort {
    item: Vec<Problem>,
}

#[async_trait]
impl AtcoderProblemsPort for DummyAtcoderProblemsPort {
    async fn fetch_problems(&self) -> Result<Vec<Problem>, ExternalError> {
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
    async fn get_problems_by_contest(
        &self,
        _contest: &str,
    ) -> Result<Vec<Problem>, RepositoryError> {
        Ok(vec![])
    }
}

#[tokio::test]
async fn usecase_calls_repo_with_converted_problems() -> Result<()> {
    let port = Arc::new(DummyAtcoderProblemsPort {
        item: vec![
            Problem {
                id: "abc234_a".into(),
                contest_code: "abc234".into(),
                problem_index: "A".into(),
                title: "A - Example".into(),
            },
            Problem {
                id: "abc234_b".into(),
                contest_code: "abc234".into(),
                problem_index: "B".into(),
                title: "B - Example".into(),
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
