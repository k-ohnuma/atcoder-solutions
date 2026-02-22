use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use domain::{
    error::repository::RepositoryError,
    ports::repository::solution::tx::{CommentRepositoryTx, SolutionRespositoryTx, SolutionTxManager, TagRepositoryTx, UnitOfWork, VoteRepositoryTx},
};
use usecase::{
    model::solution::{
        SolutionComment, SolutionDetails, SolutionError, SolutionListItem, SolutionListSort, UserSolutionListItem,
        update::UpdateSolutionInput, update_comment::UpdateCommentInput,
    },
    service::solution::SolutionService,
    solution::{
        delete::DeleteSolutionUsecase, delete_comment::DeleteCommentUsecase, unvote::UnvoteSolutionUsecase,
        update::UpdateSolutionUsecase, update_comment::UpdateCommentUsecase, vote::VoteSolutionUsecase,
    },
};
use uuid::Uuid;

struct NeverCalledTxManager;

#[async_trait]
impl SolutionTxManager for NeverCalledTxManager {
    async fn begin(&self) -> Result<Box<dyn UnitOfWork>, RepositoryError> {
        Err(RepositoryError::Unexpected(
            "begin should not be called in this test path".to_string(),
        ))
    }
}

#[derive(Clone)]
struct GateService {
    solution_exists: bool,
    comment_exists: bool,
    solution_owner: String,
    comment_owner: String,
}

#[async_trait]
impl SolutionService for GateService {
    async fn get_solutions_by_problem_id(
        &self,
        _problem_id: String,
        _sort: SolutionListSort,
    ) -> Result<Vec<SolutionListItem>, RepositoryError> {
        Ok(vec![])
    }

    async fn get_solution_by_solution_id(
        &self,
        _solution_id: Uuid,
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

    async fn user_name_exists(&self, _user_name: &str) -> Result<bool, RepositoryError> {
        Ok(false)
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
        Ok(self.solution_exists)
    }

    async fn get_comments_by_solution_id(
        &self,
        _solution_id: Uuid,
    ) -> Result<Vec<SolutionComment>, RepositoryError> {
        Ok(vec![])
    }

    async fn get_solution_user_id(&self, _solution_id: Uuid) -> Result<String, RepositoryError> {
        Ok(self.solution_owner.clone())
    }

    async fn comment_exists(&self, _comment_id: Uuid) -> Result<bool, RepositoryError> {
        Ok(self.comment_exists)
    }

    async fn get_comment_user_id(&self, _comment_id: Uuid) -> Result<String, RepositoryError> {
        Ok(self.comment_owner.clone())
    }

    async fn get_user_name_by_id(&self, _user_id: &str) -> Result<String, RepositoryError> {
        Ok("name".to_string())
    }

    async fn problem_exists(&self, _problem_id: &str) -> Result<bool, RepositoryError> {
        Ok(true)
    }
}

#[tokio::test]
async fn update_solution_forbidden_for_non_owner() -> Result<()> {
    let txm = Arc::new(NeverCalledTxManager);
    let service = Arc::new(GateService {
        solution_exists: true,
        comment_exists: true,
        solution_owner: "owner".to_string(),
        comment_owner: "owner".to_string(),
    });
    let uc = UpdateSolutionUsecase::new(txm, service);
    let input = UpdateSolutionInput {
        user_id: "attacker".to_string(),
        solution_id: Uuid::now_v7(),
        title: "title".to_string(),
        body_md: "body".to_string(),
        submit_url: String::new(),
        tags: vec![],
    };

    let err = uc.run(input).await.err().expect("should be forbidden");
    assert!(matches!(err, SolutionError::Forbidden(_)));
    Ok(())
}

#[tokio::test]
async fn delete_solution_forbidden_for_non_owner() -> Result<()> {
    let txm = Arc::new(NeverCalledTxManager);
    let service = Arc::new(GateService {
        solution_exists: true,
        comment_exists: true,
        solution_owner: "owner".to_string(),
        comment_owner: "owner".to_string(),
    });
    let uc = DeleteSolutionUsecase::new(txm, service);

    let err = uc
        .run("attacker".to_string(), Uuid::now_v7())
        .await
        .err()
        .expect("should be forbidden");
    assert!(matches!(err, SolutionError::Forbidden(_)));
    Ok(())
}

#[tokio::test]
async fn update_comment_forbidden_for_non_owner() -> Result<()> {
    let txm = Arc::new(NeverCalledTxManager);
    let service = Arc::new(GateService {
        solution_exists: true,
        comment_exists: true,
        solution_owner: "owner".to_string(),
        comment_owner: "owner".to_string(),
    });
    let uc = UpdateCommentUsecase::new(txm, service);
    let input = UpdateCommentInput {
        user_id: "attacker".to_string(),
        comment_id: Uuid::now_v7(),
        body_md: "body".to_string(),
    };

    let err = uc.run(input).await.err().expect("should be forbidden");
    assert!(matches!(err, SolutionError::Forbidden(_)));
    Ok(())
}

#[tokio::test]
async fn delete_comment_forbidden_for_non_owner() -> Result<()> {
    let txm = Arc::new(NeverCalledTxManager);
    let service = Arc::new(GateService {
        solution_exists: true,
        comment_exists: true,
        solution_owner: "owner".to_string(),
        comment_owner: "owner".to_string(),
    });
    let uc = DeleteCommentUsecase::new(txm, service);

    let err = uc
        .run("attacker".to_string(), Uuid::now_v7())
        .await
        .err()
        .expect("should be forbidden");
    assert!(matches!(err, SolutionError::Forbidden(_)));
    Ok(())
}

#[tokio::test]
async fn vote_returns_bad_request_when_solution_not_found() -> Result<()> {
    let txm = Arc::new(NeverCalledTxManager);
    let service = Arc::new(GateService {
        solution_exists: false,
        comment_exists: true,
        solution_owner: "owner".to_string(),
        comment_owner: "owner".to_string(),
    });
    let uc = VoteSolutionUsecase::new(txm, service);
    let err = uc
        .run("uid".to_string(), Uuid::now_v7())
        .await
        .err()
        .expect("should be bad request");
    assert!(matches!(err, SolutionError::BadRequest(_)));
    Ok(())
}

#[tokio::test]
async fn unvote_returns_bad_request_when_solution_not_found() -> Result<()> {
    let txm = Arc::new(NeverCalledTxManager);
    let service = Arc::new(GateService {
        solution_exists: false,
        comment_exists: true,
        solution_owner: "owner".to_string(),
        comment_owner: "owner".to_string(),
    });
    let uc = UnvoteSolutionUsecase::new(txm, service);
    let err = uc
        .run("uid".to_string(), Uuid::now_v7())
        .await
        .err()
        .expect("should be bad request");
    assert!(matches!(err, SolutionError::BadRequest(_)));
    Ok(())
}

#[tokio::test]
async fn update_solution_not_found_is_returned_before_owner_check() -> Result<()> {
    let txm = Arc::new(NeverCalledTxManager);
    let service = Arc::new(GateService {
        solution_exists: false,
        comment_exists: true,
        solution_owner: "owner".to_string(),
        comment_owner: "owner".to_string(),
    });
    let uc = UpdateSolutionUsecase::new(txm, service);
    let input = UpdateSolutionInput {
        user_id: "owner".to_string(),
        solution_id: Uuid::now_v7(),
        title: "title".to_string(),
        body_md: "body".to_string(),
        submit_url: String::new(),
        tags: vec![],
    };

    let err = uc.run(input).await.err().expect("should be not found");
    assert!(matches!(err, SolutionError::NotFound(_)));
    Ok(())
}

#[tokio::test]
async fn update_comment_not_found_is_returned_before_owner_check() -> Result<()> {
    let txm = Arc::new(NeverCalledTxManager);
    let service = Arc::new(GateService {
        solution_exists: true,
        comment_exists: false,
        solution_owner: "owner".to_string(),
        comment_owner: "owner".to_string(),
    });
    let uc = UpdateCommentUsecase::new(txm, service);
    let input = UpdateCommentInput {
        user_id: "owner".to_string(),
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
impl SolutionRespositoryTx for _NoopSolutionRepo {
    async fn create(&mut self, _s: &domain::model::solution::Solution) -> Result<Uuid, RepositoryError> {
        Err(RepositoryError::Unexpected("unused".to_string()))
    }
    async fn update(&mut self, _solution_id: Uuid, _title: &str, _body_md: &str, _submit_url: &str) -> Result<(), RepositoryError> {
        Err(RepositoryError::Unexpected("unused".to_string()))
    }
    async fn delete(&mut self, _solution_id: Uuid) -> Result<(), RepositoryError> {
        Err(RepositoryError::Unexpected("unused".to_string()))
    }
    async fn replace_tags(&mut self, _solution_id: Uuid, _tag_id: &[Uuid]) -> Result<(), RepositoryError> {
        Err(RepositoryError::Unexpected("unused".to_string()))
    }
}

struct _NoopTagRepo;
#[async_trait]
impl TagRepositoryTx for _NoopTagRepo {
    async fn upsert(&mut self, _names: &[String]) -> Result<Vec<Uuid>, RepositoryError> {
        Err(RepositoryError::Unexpected("unused".to_string()))
    }
}

struct _NoopVoteRepo;
#[async_trait]
impl VoteRepositoryTx for _NoopVoteRepo {
    async fn like(&mut self, _user_id: &str, _solution_id: Uuid) -> Result<(), RepositoryError> {
        Err(RepositoryError::Unexpected("unused".to_string()))
    }
    async fn unlike(&mut self, _user_id: &str, _solution_id: Uuid) -> Result<(), RepositoryError> {
        Err(RepositoryError::Unexpected("unused".to_string()))
    }
}

struct _NoopCommentRepo;
#[async_trait]
impl CommentRepositoryTx for _NoopCommentRepo {
    async fn create_comment(&mut self, _user_id: &str, _solution_id: Uuid, _body_md: &str) -> Result<domain::ports::repository::solution::tx::CreatedComment, RepositoryError> {
        Err(RepositoryError::Unexpected("unused".to_string()))
    }
    async fn update_comment(&mut self, _comment_id: Uuid, _body_md: &str) -> Result<domain::ports::repository::solution::tx::CreatedComment, RepositoryError> {
        Err(RepositoryError::Unexpected("unused".to_string()))
    }
    async fn delete_comment(&mut self, _comment_id: Uuid) -> Result<(), RepositoryError> {
        Err(RepositoryError::Unexpected("unused".to_string()))
    }
}

#[allow(dead_code)]
struct _NoopUow {
    s: _NoopSolutionRepo,
    t: _NoopTagRepo,
    v: _NoopVoteRepo,
    c: _NoopCommentRepo,
}

#[async_trait]
impl UnitOfWork for _NoopUow {
    fn solutions(&mut self) -> &mut dyn SolutionRespositoryTx {
        &mut self.s
    }
    fn tags(&mut self) -> &mut dyn TagRepositoryTx {
        &mut self.t
    }
    fn votes(&mut self) -> &mut dyn VoteRepositoryTx {
        &mut self.v
    }
    fn comments(&mut self) -> &mut dyn CommentRepositoryTx {
        &mut self.c
    }
    async fn commit(self: Box<Self>) -> Result<(), RepositoryError> {
        let _ = self;
        Ok(())
    }
    async fn rollback(self: Box<Self>) -> Result<(), RepositoryError> {
        let _ = self;
        Ok(())
    }
}
