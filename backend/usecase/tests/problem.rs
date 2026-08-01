use std::sync::{Arc, Mutex};

use anyhow::Result;
use async_trait::async_trait;
use domain::{
    error::{external::ExternalError, repository::RepositoryError},
    model::problem::Problem,
    ports::{
        external::atcoder_problems::AtcoderProblemsPort,
        repository::problem::{
            ProblemRepository,
            tx::{ProblemRepositoryTx, ProblemTxManager, ProblemUnitOfWork},
        },
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

    async fn fetch_difficulty(&self, _problem_id: &str) -> Result<Option<i32>, ExternalError> {
        Ok(None)
    }
}

#[derive(Default)]
struct TxCalls {
    contests: Vec<(String, String)>,
    problems: Vec<Problem>,
    contest_upsert_calls: usize,
    problem_upsert_calls: usize,
    commits: usize,
}

struct DummyProblemUow {
    shared: Arc<Mutex<TxCalls>>,
}

#[async_trait]
impl ProblemRepositoryTx for DummyProblemUow {
    async fn upsert_contests(
        &mut self,
        contests: &[(String, String)],
    ) -> Result<(), RepositoryError> {
        let mut calls = self.shared.lock().unwrap();
        calls.contest_upsert_calls += 1;
        calls.contests.extend(contests.iter().cloned());
        Ok(())
    }

    async fn upsert_problems(&mut self, problems: &[Problem]) -> Result<(), RepositoryError> {
        let mut calls = self.shared.lock().unwrap();
        calls.problem_upsert_calls += 1;
        calls.problems.extend(problems.iter().cloned());
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

struct DummyProblemRepository;

#[async_trait]
impl ProblemRepository for DummyProblemRepository {
    async fn get_problem_ids_with_difficulty(
        &self,
        _problem_ids: &[String],
    ) -> Result<Vec<String>, RepositoryError> {
        Ok(vec![])
    }

    async fn get_contest_codes_by_series(
        &self,
        _series: domain::model::problem::ContestSeries,
        _limit: i64,
        _offset: i64,
    ) -> Result<Vec<String>, RepositoryError> {
        Ok(vec![])
    }

    async fn get_problems_by_contest_codes(
        &self,
        _contest_codes: &[String],
    ) -> Result<Vec<Problem>, RepositoryError> {
        Ok(vec![])
    }

    async fn search_problems_by_contest_series(
        &self,
        _series: domain::model::problem::ContestSeries,
        _query: &str,
    ) -> Result<Vec<Problem>, RepositoryError> {
        Ok(vec![])
    }

    async fn get_problem_by_id(&self, problem_id: &str) -> Result<Problem, RepositoryError> {
        Err(RepositoryError::NotFound(problem_id.to_string()))
    }

    async fn problem_exists(&self, _problem_id: &str) -> Result<bool, RepositoryError> {
        Ok(false)
    }

    async fn get_problems_by_contest(
        &self,
        _contest: &str,
    ) -> Result<Vec<Problem>, RepositoryError> {
        Ok(vec![])
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
                difficulty: None,
            },
            Problem {
                id: "abc234_b".into(),
                contest_code: "abc234".into(),
                problem_index: "B".into(),
                title: "B - Example".into(),
                difficulty: None,
            },
            Problem {
                id: "awc001_a".into(),
                contest_code: "awc001".into(),
                problem_index: "A".into(),
                title: "AWC - Example".into(),
                difficulty: None,
            },
            Problem {
                id: "tessoku_a1".into(),
                contest_code: "tessoku-book".into(),
                problem_index: "A1".into(),
                title: "Tessoku - Example".into(),
                difficulty: None,
            },
        ],
    });
    let calls = Arc::new(Mutex::new(TxCalls::default()));
    let txm = Arc::new(DummyProblemTxManager {
        shared: calls.clone(),
    });
    let repo = Arc::new(DummyProblemRepository);

    let uc = ImportProblemsUsecase::new(port, repo, txm);
    uc.run().await.unwrap();

    let calls = calls.lock().unwrap();
    assert_eq!(calls.commits, 1);
    assert_eq!(calls.contest_upsert_calls, 1);
    assert_eq!(calls.problem_upsert_calls, 1);
    assert_eq!(calls.problems.len(), 4);
    assert_eq!(calls.contests.len(), 3);
    assert!(calls.contests.contains(&("abc234".into(), "ABC".into())));
    assert!(calls.contests.contains(&("awc001".into(), "AWC".into())));
    assert!(
        calls
            .contests
            .contains(&("tessoku-book".into(), "OTHER".into()))
    );

    let sent = &calls.problems[0];
    assert_eq!(sent.id, "abc234_a");
    assert_eq!(sent.title, "A - Example");
    Ok(())
}
