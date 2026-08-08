use domain::{
    model::{
        solution::{comment_id::CommentId, id::SolutionId},
        user::id::UserId,
    },
    ports::repository::solution::SolutionRepository,
};

use crate::error::SolutionError;

pub async fn ensure_solution_owner(
    repository: &dyn SolutionRepository,
    solution_id: SolutionId,
    user_id: &UserId,
    forbidden_message: &str,
) -> Result<(), SolutionError> {
    let owner_user_id = repository
        .get_solution_user_id(solution_id)
        .await?
        .ok_or_else(|| SolutionError::NotFound("solution not found".to_string()))?;
    if &owner_user_id != user_id {
        return Err(SolutionError::Forbidden(forbidden_message.to_string()));
    }

    Ok(())
}

pub async fn ensure_comment_owner(
    repository: &dyn SolutionRepository,
    comment_id: CommentId,
    user_id: &UserId,
    forbidden_message: &str,
) -> Result<(), SolutionError> {
    let owner_user_id = repository
        .get_comment_user_id(comment_id)
        .await?
        .ok_or_else(|| SolutionError::NotFound("comment not found".to_string()))?;
    if &owner_user_id != user_id {
        return Err(SolutionError::Forbidden(forbidden_message.to_string()));
    }

    Ok(())
}
