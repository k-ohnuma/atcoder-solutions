use std::sync::{Arc, Mutex};

use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use domain::{
    error::repository::RepositoryError,
    model::{
        problem::{ContestSeries, Problem},
        user::User,
    },
    ports::repository::{
        problem::ProblemRepository, solution::SolutionRepository, user::UserRepository,
    },
    read_model::solution::{
        SolutionComment, SolutionDetails, SolutionListItem, SolutionListSort, UserSolutionListItem,
    },
};
use usecase::{
    error::SolutionError,
    model::solution::list::{
        GetLatestSolutionsInput, GetSolutionsByProblemIdInput, GetSolutionsByUserNameInput,
    },
    solution::{
        get_by_problem_id::GetSolutionsByProblemIdUsecase,
        get_by_user_name::GetSolutionsByUserNameUsecase, get_latest::GetLatestSolutionsUsecase,
    },
};
use uuid::Uuid;

struct DummySolutionRepository {
    last_problem_sort: Mutex<Option<SolutionListSort>>,
    last_user_sort: Mutex<Option<SolutionListSort>>,
}

impl DummySolutionRepository {
    fn new() -> Self {
        Self {
            last_problem_sort: Mutex::new(None),
            last_user_sort: Mutex::new(None),
        }
    }
}

struct DummyProblemRepository {
    exists: bool,
}

struct DummyUserRepository {
    exists: bool,
}

#[async_trait]
impl SolutionRepository for DummySolutionRepository {
    async fn delete_solution(&self, _solution_id: Uuid) -> Result<(), RepositoryError> {
        Ok(())
    }

    async fn vote_solution(
        &self,
        _user_id: &str,
        _solution_id: Uuid,
    ) -> Result<(), RepositoryError> {
        Ok(())
    }

    async fn unvote_solution(
        &self,
        _user_id: &str,
        _solution_id: Uuid,
    ) -> Result<(), RepositoryError> {
        Ok(())
    }

    async fn delete_comment(&self, _comment_id: Uuid) -> Result<(), RepositoryError> {
        Ok(())
    }

    async fn create_comment(
        &self,
        user_id: &str,
        solution_id: Uuid,
        body_md: &str,
    ) -> Result<SolutionComment, RepositoryError> {
        Ok(SolutionComment {
            id: Uuid::now_v7(),
            user_id: user_id.to_string(),
            user_name: "alice".to_string(),
            solution_id,
            body_md: body_md.to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    async fn update_comment(
        &self,
        _comment_id: Uuid,
        body_md: &str,
    ) -> Result<SolutionComment, RepositoryError> {
        Ok(SolutionComment {
            id: Uuid::now_v7(),
            user_id: "uid".to_string(),
            user_name: "alice".to_string(),
            solution_id: Uuid::now_v7(),
            body_md: body_md.to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    async fn get_latest_solutions(
        &self,
        _limit: Option<i32>,
    ) -> Result<Vec<SolutionListItem>, RepositoryError> {
        Ok(vec![SolutionListItem {
            id: Uuid::now_v7(),
            title: "latest".to_string(),
            problem_id: "abc100_a".to_string(),
            user_id: "uid".to_string(),
            user_name: "alice".to_string(),
            votes_count: 1,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }])
    }

    async fn get_solutions_by_problem_id(
        &self,
        _problem_id: String,
        sort: SolutionListSort,
        _limit: Option<i32>,
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
}

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
        _series: ContestSeries,
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
        _series: ContestSeries,
        _query: &str,
    ) -> Result<Vec<Problem>, RepositoryError> {
        Ok(vec![])
    }

    async fn get_problem_by_id(&self, problem_id: &str) -> Result<Problem, RepositoryError> {
        Err(RepositoryError::NotFound(problem_id.to_string()))
    }

    async fn problem_exists(&self, _problem_id: &str) -> Result<bool, RepositoryError> {
        Ok(self.exists)
    }

    async fn get_problems_by_contest(
        &self,
        _contest: &str,
    ) -> Result<Vec<Problem>, RepositoryError> {
        Ok(vec![])
    }
}

#[async_trait]
impl UserRepository for DummyUserRepository {
    async fn create_user(&self, _user: User) -> Result<(), RepositoryError> {
        Ok(())
    }

    async fn find_by_uid(&self, uid: &str) -> Result<User, RepositoryError> {
        Err(RepositoryError::NotFound(uid.to_string()))
    }

    async fn user_name_exists(&self, _user_name: &str) -> Result<bool, RepositoryError> {
        Ok(self.exists)
    }

    async fn delete_by_uid(&self, _uid: &str) -> Result<(), RepositoryError> {
        Ok(())
    }

    async fn revoke_tokens_by_uid(&self, _uid: &str) -> Result<(), RepositoryError> {
        Ok(())
    }

    async fn is_token_revoked(&self, _uid: &str, _issued_at: i64) -> Result<bool, RepositoryError> {
        Ok(false)
    }
}

#[tokio::test]
async fn get_solutions_by_problem_id_rejects_blank_problem_id() -> Result<()> {
    let err = GetSolutionsByProblemIdInput::try_new("   ".to_string(), None, None)
        .err()
        .expect("blank problem id should be bad request");
    assert!(matches!(err, SolutionError::BadRequest(_)));

    Ok(())
}

#[tokio::test]
async fn get_solutions_by_problem_id_returns_not_found_for_unknown_problem() -> Result<()> {
    let repository = Arc::new(DummySolutionRepository::new());
    let problem_repository = Arc::new(DummyProblemRepository { exists: false });
    let uc = GetSolutionsByProblemIdUsecase::new(repository, problem_repository);
    let input =
        GetSolutionsByProblemIdInput::try_new("abc100_a".to_string(), None, None).expect("valid");

    let err = uc
        .run(input)
        .await
        .err()
        .expect("unknown problem should be not found");
    assert!(matches!(err, SolutionError::NotFound(_)));

    Ok(())
}

#[tokio::test]
async fn get_solutions_by_problem_id_passes_sort_to_repository() -> Result<()> {
    let repository = Arc::new(DummySolutionRepository::new());
    let problem_repository = Arc::new(DummyProblemRepository { exists: true });
    let uc = GetSolutionsByProblemIdUsecase::new(repository.clone(), problem_repository);
    let input = GetSolutionsByProblemIdInput::try_new(
        "abc100_a".to_string(),
        Some("votes".to_string()),
        None,
    )
    .expect("valid");

    let result = uc.run(input).await?;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].votes_count, 3);

    let sort = *repository.last_problem_sort.lock().unwrap();
    assert!(matches!(sort, Some(SolutionListSort::Votes)));
    Ok(())
}

#[tokio::test]
async fn get_latest_solutions_returns_items() -> Result<()> {
    let repository = Arc::new(DummySolutionRepository::new());
    let uc = GetLatestSolutionsUsecase::new(repository);
    let input = GetLatestSolutionsInput::try_new(None, Some(10)).expect("valid");

    let result = uc.run(input).await?;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].title, "latest");

    Ok(())
}

#[tokio::test]
async fn get_solutions_by_user_name_returns_not_found_for_unknown_user() -> Result<()> {
    let repository = Arc::new(DummySolutionRepository::new());
    let user_repository = Arc::new(DummyUserRepository { exists: false });
    let uc = GetSolutionsByUserNameUsecase::new(repository, user_repository);
    let input = GetSolutionsByUserNameInput::try_new("alice".to_string(), None).expect("valid");

    let err = uc
        .run(input)
        .await
        .err()
        .expect("unknown user should be not found");
    assert!(matches!(err, SolutionError::NotFound(_)));
    Ok(())
}

#[tokio::test]
async fn get_solutions_by_user_name_passes_sort_to_repository() -> Result<()> {
    let repository = Arc::new(DummySolutionRepository::new());
    let user_repository = Arc::new(DummyUserRepository { exists: true });
    let uc = GetSolutionsByUserNameUsecase::new(repository.clone(), user_repository);
    let input =
        GetSolutionsByUserNameInput::try_new("alice".to_string(), Some("votes".to_string()))
            .expect("valid");

    let result = uc.run(input).await?;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].problem_title, "A - Sample");

    let sort = *repository.last_user_sort.lock().unwrap();
    assert!(matches!(sort, Some(SolutionListSort::Votes)));
    Ok(())
}
