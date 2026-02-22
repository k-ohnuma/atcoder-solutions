use std::sync::{Arc, Mutex};

use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use domain::error::repository::RepositoryError;
use usecase::{
    model::solution::{SolutionComment, SolutionDetails, SolutionError, SolutionListItem, SolutionListSort, UserSolutionListItem},
    service::solution::SolutionService,
    solution::{get_by_problem_id::GetSolutionsByProblemIdUsecase, get_by_user_name::GetSolutionsByUserNameUsecase},
};
use uuid::Uuid;

struct DummySolutionService {
    problem_exists: bool,
    user_exists: bool,
    last_problem_sort: Mutex<Option<SolutionListSort>>,
    last_user_sort: Mutex<Option<SolutionListSort>>,
}

impl DummySolutionService {
    fn new(problem_exists: bool, user_exists: bool) -> Self {
        Self {
            problem_exists,
            user_exists,
            last_problem_sort: Mutex::new(None),
            last_user_sort: Mutex::new(None),
        }
    }
}

#[async_trait]
impl SolutionService for DummySolutionService {
    async fn get_solutions_by_problem_id(
        &self,
        _problem_id: String,
        sort: SolutionListSort,
    ) -> Result<Vec<SolutionListItem>, RepositoryError> {
        *self.last_problem_sort.lock().unwrap() = Some(sort);
        Ok(vec![SolutionListItem {
            id: Uuid::now_v7(),
            title: "t".to_string(),
            problem_id: "abc100_a".to_string(),
            user_id: "uid".to_string(),
            user_name: "alice".to_string(),
            votes_count: 3,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }])
    }

    async fn get_solution_by_solution_id(
        &self,
        _solution_id: Uuid,
    ) -> Result<SolutionDetails, RepositoryError> {
        Err(RepositoryError::Unexpected(
            "not used in this test".to_string(),
        ))
    }

    async fn get_solutions_by_user_name(
        &self,
        _user_name: String,
        sort: SolutionListSort,
    ) -> Result<Vec<UserSolutionListItem>, RepositoryError> {
        *self.last_user_sort.lock().unwrap() = Some(sort);
        Ok(vec![UserSolutionListItem {
            id: Uuid::now_v7(),
            title: "t".to_string(),
            problem_id: "abc100_a".to_string(),
            problem_title: "A - Sample".to_string(),
            user_id: "uid".to_string(),
            user_name: "alice".to_string(),
            votes_count: 5,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }])
    }

    async fn user_name_exists(&self, _user_name: &str) -> Result<bool, RepositoryError> {
        Ok(self.user_exists)
    }

    async fn get_solution_votes_count(&self, _solution_id: Uuid) -> Result<i64, RepositoryError> {
        Ok(0)
    }

    async fn has_user_voted_solution(
        &self,
        _user_id: String,
        _solution_id: Uuid,
    ) -> Result<bool, RepositoryError> {
        Ok(false)
    }

    async fn solution_exists(&self, _solution_id: Uuid) -> Result<bool, RepositoryError> {
        Ok(false)
    }

    async fn get_comments_by_solution_id(
        &self,
        _solution_id: Uuid,
    ) -> Result<Vec<SolutionComment>, RepositoryError> {
        Ok(vec![])
    }

    async fn get_solution_user_id(&self, _solution_id: Uuid) -> Result<String, RepositoryError> {
        Ok(String::new())
    }

    async fn comment_exists(&self, _comment_id: Uuid) -> Result<bool, RepositoryError> {
        Ok(false)
    }

    async fn get_comment_user_id(&self, _comment_id: Uuid) -> Result<String, RepositoryError> {
        Ok(String::new())
    }

    async fn get_user_name_by_id(&self, _user_id: &str) -> Result<String, RepositoryError> {
        Ok(String::new())
    }

    async fn problem_exists(&self, _problem_id: &str) -> Result<bool, RepositoryError> {
        Ok(self.problem_exists)
    }
}

#[tokio::test]
async fn get_solutions_by_problem_id_rejects_blank_problem_id() -> Result<()> {
    let service = Arc::new(DummySolutionService::new(true, true));
    let uc = GetSolutionsByProblemIdUsecase::new(service);

    let err = uc
        .run("   ".to_string(), SolutionListSort::Latest)
        .await
        .err()
        .expect("blank problem id should be bad request");
    assert!(matches!(err, SolutionError::BadRequest(_)));

    Ok(())
}

#[tokio::test]
async fn get_solutions_by_problem_id_returns_not_found_for_unknown_problem() -> Result<()> {
    let service = Arc::new(DummySolutionService::new(false, true));
    let uc = GetSolutionsByProblemIdUsecase::new(service);

    let err = uc
        .run("abc100_a".to_string(), SolutionListSort::Latest)
        .await
        .err()
        .expect("unknown problem should be not found");
    assert!(matches!(err, SolutionError::NotFound(_)));

    Ok(())
}

#[tokio::test]
async fn get_solutions_by_problem_id_passes_sort_to_service() -> Result<()> {
    let service = Arc::new(DummySolutionService::new(true, true));
    let uc = GetSolutionsByProblemIdUsecase::new(service.clone());

    let result = uc
        .run("abc100_a".to_string(), SolutionListSort::Votes)
        .await?;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].votes_count, 3);

    let sort = *service.last_problem_sort.lock().unwrap();
    assert!(matches!(sort, Some(SolutionListSort::Votes)));
    Ok(())
}

#[tokio::test]
async fn get_solutions_by_user_name_returns_not_found_for_unknown_user() -> Result<()> {
    let service = Arc::new(DummySolutionService::new(true, false));
    let uc = GetSolutionsByUserNameUsecase::new(service);

    let err = uc
        .run("alice".to_string(), SolutionListSort::Latest)
        .await
        .err()
        .expect("unknown user should be not found");
    assert!(matches!(err, SolutionError::NotFound(_)));
    Ok(())
}

#[tokio::test]
async fn get_solutions_by_user_name_passes_sort_to_service() -> Result<()> {
    let service = Arc::new(DummySolutionService::new(true, true));
    let uc = GetSolutionsByUserNameUsecase::new(service.clone());

    let result = uc
        .run("alice".to_string(), SolutionListSort::Votes)
        .await?;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].problem_title, "A - Sample");

    let sort = *service.last_user_sort.lock().unwrap();
    assert!(matches!(sort, Some(SolutionListSort::Votes)));
    Ok(())
}
