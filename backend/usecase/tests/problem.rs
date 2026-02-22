use std::sync::{Arc, Mutex};

use anyhow::Result;
use async_trait::async_trait;
use domain::{
    error::{external::ExternalError, repository::RepositoryError},
    model::problem::Problem,
    ports::{
        external::atcoder_problems::AtcoderProblemsPort,
        repository::problem::tx::{ProblemRepositoryTx, ProblemTxManager, ProblemUnitOfWork},
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

#[derive(Default)]
struct TxCalls {
    contests: Vec<(String, String)>,
    problems: Vec<Problem>,
    commits: usize,
}

struct DummyProblemUow {
    shared: Arc<Mutex<TxCalls>>,
}

#[async_trait]
impl ProblemRepositoryTx for DummyProblemUow {
    async fn upsert_contest(
        &mut self,
        contest_code: &str,
        series_code: &str,
    ) -> Result<(), RepositoryError> {
        self.shared
            .lock()
            .unwrap()
            .contests
            .push((contest_code.to_string(), series_code.to_string()));
        Ok(())
    }

    async fn upsert_problem(
        &mut self,
        problem_id: &str,
        contest_code: &str,
        problem_index: &str,
        title: &str,
    ) -> Result<(), RepositoryError> {
        self.shared.lock().unwrap().problems.push(Problem {
            id: problem_id.to_string(),
            contest_code: contest_code.to_string(),
            problem_index: problem_index.to_string(),
            title: title.to_string(),
        });
        Ok(())
    }
}

#[async_trait]
impl ProblemUnitOfWork for DummyProblemUow {
    fn problems(&mut self) -> &mut dyn ProblemRepositoryTx {
        self
    }

    async fn commit(self: Box<Self>) -> Result<(), RepositoryError> {
        self.shared.lock().unwrap().commits += 1;
        Ok(())
    }

    async fn rollback(self: Box<Self>) -> Result<(), RepositoryError> {
        Ok(())
    }
}

struct DummyProblemTxManager {
    shared: Arc<Mutex<TxCalls>>,
}

#[async_trait]
impl ProblemTxManager for DummyProblemTxManager {
    async fn begin(&self) -> Result<Box<dyn ProblemUnitOfWork>, RepositoryError> {
        Ok(Box::new(DummyProblemUow {
            shared: self.shared.clone(),
        }))
    }
}

#[tokio::test]
async fn usecase_imports_problems_in_single_uow() -> Result<()> {
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
    let calls = Arc::new(Mutex::new(TxCalls::default()));
    let txm = Arc::new(DummyProblemTxManager {
        shared: calls.clone(),
    });

    let uc = ImportProblemsUsecase::new(port, txm);
    uc.run().await.unwrap();

    let calls = calls.lock().unwrap();
    assert_eq!(calls.commits, 1);
    assert_eq!(calls.problems.len(), 2);
    assert_eq!(calls.contests.len(), 1);
    assert_eq!(calls.contests[0].0, "abc234");
    assert_eq!(calls.contests[0].1, "ABC");

    let sent = &calls.problems[0];
    assert_eq!(sent.id, "abc234_a");
    assert_eq!(sent.title, "A - Example");
    Ok(())
}
