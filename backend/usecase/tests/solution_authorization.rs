use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use domain::{
    error::repository::RepositoryError,
    model::solution::{
        content::{SolutionBody, SolutionTitle, SubmitUrl},
        id::SolutionId,
        tags::Tags,
    },
    model::user::id::UserId,
    ports::repository::solution::{
        SolutionRepository,
        tx::{SolutionRepositoryTx, SolutionTxManager, SolutionUnitOfWork, TagRepositoryTx},
    },
    read_model::solution::{
        SolutionComment, SolutionDetails, SolutionListItem, SolutionListSort, UserSolutionListItem,
    },
};
use usecase::{
    error::SolutionError,
    model::solution::{
        create_comment::CreateCommentInput, update::UpdateSolutionInput,
        update_comment::UpdateCommentInput,
    },
    solution::{
        create_comment::CreateCommentUsecase, delete::DeleteSolutionUsecase,
        delete_comment::DeleteCommentUsecase, unvote::UnvoteSolutionUsecase,
        update::UpdateSolutionUsecase, update_comment::UpdateCommentUsecase,
        vote::VoteSolutionUsecase,
    },
};
use uuid::Uuid;

fn user_id(value: &str) -> UserId {
    UserId::try_new(value.to_string()).expect("valid user ID")
}

struct NeverCalledTxManager;

#[async_trait]
impl SolutionTxManager for NeverCalledTxManager {
    async fn begin(&self) -> Result<Box<dyn SolutionUnitOfWork>, RepositoryError> {
        Err(RepositoryError::Unexpected(
            "begin should not be called in this test path".to_string(),
        ))
    }
}

#[derive(Clone)]
struct GateRepository {
    solution_exists: bool,
    comment_exists: bool,
    solution_owner: String,
    comment_owner: String,
}

#[async_trait]
impl SolutionRepository for GateRepository {
    async fn delete_solution(&self, _solution_id: SolutionId) -> Result<(), RepositoryError> {
        Ok(())
    }

    async fn vote_solution(
        &self,
        _user_id: &UserId,
        _solution_id: SolutionId,
    ) -> Result<(), RepositoryError> {
        Ok(())
    }

    async fn unvote_solution(
        &self,
        _user_id: &UserId,
        _solution_id: SolutionId,
    ) -> Result<(), RepositoryError> {
        Ok(())
    }

    async fn delete_comment(&self, _comment_id: Uuid) -> Result<(), RepositoryError> {
        Ok(())
    }

    async fn create_comment(
        &self,
        user_id: &UserId,
        solution_id: SolutionId,
        body_md: &str,
    ) -> Result<SolutionComment, RepositoryError> {
        Ok(SolutionComment {
            id: Uuid::now_v7(),
            user_id: user_id.clone(),
            user_name: "name".to_string(),
            solution_id,
            body_md: body_md.to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }

    async fn update_comment(
        &self,
        _comment_id: Uuid,
        body_md: &str,
    ) -> Result<SolutionComment, RepositoryError> {
        Ok(SolutionComment {
            id: Uuid::now_v7(),
            user_id: user_id(&self.comment_owner),
            user_name: "name".to_string(),
            solution_id: Uuid::now_v7().into(),
            body_md: body_md.to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }

    async fn get_latest_solutions(
        &self,
        _limit: Option<i32>,
    ) -> Result<Vec<SolutionListItem>, RepositoryError> {
        Ok(vec![])
    }

    async fn get_solutions_by_problem_id(
        &self,
        _problem_id: domain::model::problem::id::ProblemId,
        _sort: SolutionListSort,
        _limit: Option<i32>,
    ) -> Result<Vec<SolutionListItem>, RepositoryError> {
        Ok(vec![])
    }

    async fn get_solution_by_solution_id(
        &self,
        _solution_id: SolutionId,
    ) -> Result<SolutionDetails, RepositoryError> {
        Err(RepositoryError::Unexpected("unused".to_string()))
    }

    async fn get_solutions_by_user_name(
        &self,
        _user_name: String,
        _sort: SolutionListSort,
    ) -> Result<Vec<UserSolutionListItem>, RepositoryError> {
        Ok(vec![])
    }

    async fn get_solution_votes_count(
        &self,
        _solution_id: SolutionId,
    ) -> Result<i64, RepositoryError> {
        Ok(0)
    }

    async fn has_user_voted_solution(
        &self,
        _user_id: &UserId,
        _solution_id: SolutionId,
    ) -> Result<bool, RepositoryError> {
        Ok(false)
    }

    async fn solution_exists(&self, _solution_id: SolutionId) -> Result<bool, RepositoryError> {
        Ok(self.solution_exists)
    }

    async fn get_comments_by_solution_id(
        &self,
        _solution_id: SolutionId,
    ) -> Result<Vec<SolutionComment>, RepositoryError> {
        Ok(vec![])
    }

    async fn get_solution_user_id(
        &self,
        _solution_id: SolutionId,
    ) -> Result<UserId, RepositoryError> {
        Ok(user_id(&self.solution_owner))
    }

    async fn comment_exists(&self, _comment_id: Uuid) -> Result<bool, RepositoryError> {
        Ok(self.comment_exists)
    }

    async fn get_comment_user_id(&self, _comment_id: Uuid) -> Result<UserId, RepositoryError> {
        Ok(user_id(&self.comment_owner))
    }
}

#[tokio::test]
async fn update_solution_forbidden_for_non_owner() -> Result<()> {
    let txm = Arc::new(NeverCalledTxManager);
    let repository = Arc::new(GateRepository {
        solution_exists: true,
        comment_exists: true,
        solution_owner: "owner".to_string(),
        comment_owner: "owner".to_string(),
    });
    let uc = UpdateSolutionUsecase::new(txm, repository);
    let input = UpdateSolutionInput {
        user_id: user_id("attacker"),
        solution_id: Uuid::now_v7().into(),
        title: "title".to_string(),
        body_md: "body".to_string(),
        submit_url: String::new(),
        tags: vec![],
    };

    let err = uc.run(input).await.expect_err("should be forbidden");
    assert!(matches!(err, SolutionError::Forbidden(_)));
    Ok(())
}

#[tokio::test]
async fn delete_solution_forbidden_for_non_owner() -> Result<()> {
    let repository = Arc::new(GateRepository {
        solution_exists: true,
        comment_exists: true,
        solution_owner: "owner".to_string(),
        comment_owner: "owner".to_string(),
    });
    let uc = DeleteSolutionUsecase::new(repository);

    let err = uc
        .run(user_id("attacker"), Uuid::now_v7().into())
        .await
        .expect_err("should be forbidden");
    assert!(matches!(err, SolutionError::Forbidden(_)));
    Ok(())
}

#[tokio::test]
async fn update_comment_forbidden_for_non_owner() -> Result<()> {
    let repository = Arc::new(GateRepository {
        solution_exists: true,
        comment_exists: true,
        solution_owner: "owner".to_string(),
        comment_owner: "owner".to_string(),
    });
    let uc = UpdateCommentUsecase::new(repository);
    let input = UpdateCommentInput {
        user_id: user_id("attacker"),
        comment_id: Uuid::now_v7(),
        body_md: "body".to_string(),
    };

    let err = uc.run(input).await.err().expect("should be forbidden");
    assert!(matches!(err, SolutionError::Forbidden(_)));
    Ok(())
}

#[tokio::test]
async fn delete_comment_forbidden_for_non_owner() -> Result<()> {
    let repository = Arc::new(GateRepository {
        solution_exists: true,
        comment_exists: true,
        solution_owner: "owner".to_string(),
        comment_owner: "owner".to_string(),
    });
    let uc = DeleteCommentUsecase::new(repository);

    let err = uc
        .run(user_id("attacker"), Uuid::now_v7())
        .await
        .expect_err("should be forbidden");
    assert!(matches!(err, SolutionError::Forbidden(_)));
    Ok(())
}

#[tokio::test]
async fn vote_returns_not_found_when_solution_not_found() -> Result<()> {
    let repository = Arc::new(GateRepository {
        solution_exists: false,
        comment_exists: true,
        solution_owner: "owner".to_string(),
        comment_owner: "owner".to_string(),
    });
    let uc = VoteSolutionUsecase::new(repository);
    let err = uc
        .run(user_id("uid"), Uuid::now_v7().into())
        .await
        .expect_err("should be not found");
    assert!(matches!(err, SolutionError::NotFound(_)));
    Ok(())
}

#[tokio::test]
async fn unvote_returns_not_found_when_solution_not_found() -> Result<()> {
    let repository = Arc::new(GateRepository {
        solution_exists: false,
        comment_exists: true,
        solution_owner: "owner".to_string(),
        comment_owner: "owner".to_string(),
    });
    let uc = UnvoteSolutionUsecase::new(repository);
    let err = uc
        .run(user_id("uid"), Uuid::now_v7().into())
        .await
        .expect_err("should be not found");
    assert!(matches!(err, SolutionError::NotFound(_)));
    Ok(())
}

#[tokio::test]
async fn create_comment_returns_not_found_when_solution_not_found() -> Result<()> {
    let repository = Arc::new(GateRepository {
        solution_exists: false,
        comment_exists: true,
        solution_owner: "owner".to_string(),
        comment_owner: "owner".to_string(),
    });
    let uc = CreateCommentUsecase::new(repository);
    let input = CreateCommentInput {
        user_id: user_id("uid"),
        solution_id: Uuid::now_v7().into(),
        body_md: "body".to_string(),
    };

    let err = uc.run(input).await.err().expect("should be not found");
    assert!(matches!(err, SolutionError::NotFound(_)));
    Ok(())
}

#[tokio::test]
async fn update_solution_not_found_is_returned_before_owner_check() -> Result<()> {
    let txm = Arc::new(NeverCalledTxManager);
    let repository = Arc::new(GateRepository {
        solution_exists: false,
        comment_exists: true,
        solution_owner: "owner".to_string(),
        comment_owner: "owner".to_string(),
    });
    let uc = UpdateSolutionUsecase::new(txm, repository);
    let input = UpdateSolutionInput {
        user_id: user_id("owner"),
        solution_id: Uuid::now_v7().into(),
        title: "title".to_string(),
        body_md: "body".to_string(),
        submit_url: String::new(),
        tags: vec![],
    };

    let err = uc.run(input).await.expect_err("should be not found");
    assert!(matches!(err, SolutionError::NotFound(_)));
    Ok(())
}

#[tokio::test]
async fn update_comment_not_found_is_returned_before_owner_check() -> Result<()> {
    let repository = Arc::new(GateRepository {
        solution_exists: true,
        comment_exists: false,
        solution_owner: "owner".to_string(),
        comment_owner: "owner".to_string(),
    });
    let uc = UpdateCommentUsecase::new(repository);
    let input = UpdateCommentInput {
        user_id: user_id("owner"),
        comment_id: Uuid::now_v7(),
        body_md: "body".to_string(),
    };

    let err = uc.run(input).await.err().expect("should be not found");
    assert!(matches!(err, SolutionError::NotFound(_)));
    Ok(())
}

// Unused trait items for the no-op tx manager path.
struct _NoopSolutionRepo;
#[async_trait]
impl SolutionRepositoryTx for _NoopSolutionRepo {
    async fn create(
        &mut self,
        _solution: &domain::model::solution::Solution,
    ) -> Result<SolutionId, RepositoryError> {
        Err(RepositoryError::Unexpected("unused".to_string()))
    }
    async fn update(
        &mut self,
        _solution_id: SolutionId,
        _title: &SolutionTitle,
        _body: &SolutionBody,
        _submit_url: &SubmitUrl,
    ) -> Result<(), RepositoryError> {
        Err(RepositoryError::Unexpected("unused".to_string()))
    }
    async fn replace_tags(
        &mut self,
        _solution_id: SolutionId,
        _tag_ids: &[Uuid],
    ) -> Result<(), RepositoryError> {
        Err(RepositoryError::Unexpected("unused".to_string()))
    }
}

struct _NoopTagRepo;
#[async_trait]
impl TagRepositoryTx for _NoopTagRepo {
    async fn upsert(&mut self, _tags: &Tags) -> Result<Vec<Uuid>, RepositoryError> {
        Err(RepositoryError::Unexpected("unused".to_string()))
    }
}

#[allow(dead_code)]
struct _NoopUow {
    s: _NoopSolutionRepo,
    t: _NoopTagRepo,
}

#[async_trait]
impl SolutionUnitOfWork for _NoopUow {
    fn solutions(&mut self) -> &mut dyn SolutionRepositoryTx {
        &mut self.s
    }
    fn tags(&mut self) -> &mut dyn TagRepositoryTx {
        &mut self.t
    }
    async fn commit(self: Box<Self>) -> Result<(), RepositoryError> {
        let _ = self;
        Ok(())
    }
}
