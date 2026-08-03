use domain::{
    model::{solution::id::SolutionId, user::id::UserId},
    ports::repository::solution::SolutionRepository,
};
use uuid::Uuid;

use crate::error::SolutionError;

pub async fn ensure_solution_owner(
    repository: &dyn SolutionRepository,
    solution_id: SolutionId,
    user_id: &UserId,
    forbidden_message: &str,
) -> Result<(), SolutionError> {
    let exists = repository.solution_exists(solution_id).await?;
    if !exists {
        return Err(SolutionError::NotFound("solution not found".to_string()));
    }

    let owner_user_id = repository.get_solution_user_id(solution_id).await?;
    if &owner_user_id != user_id {
        return Err(SolutionError::Forbidden(forbidden_message.to_string()));
    }

    Ok(())
}

pub async fn ensure_comment_owner(
    repository: &dyn SolutionRepository,
    comment_id: Uuid,
    user_id: &UserId,
    forbidden_message: &str,
) -> Result<(), SolutionError> {
    let exists = repository.comment_exists(comment_id).await?;
    if !exists {
        return Err(SolutionError::NotFound("comment not found".to_string()));
    }

    let owner_user_id = repository.get_comment_user_id(comment_id).await?;
    if &owner_user_id != user_id {
        return Err(SolutionError::Forbidden(forbidden_message.to_string()));
    }

    Ok(())
}
