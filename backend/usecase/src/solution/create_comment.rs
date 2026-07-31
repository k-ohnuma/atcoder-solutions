use std::sync::Arc;

use derive_new::new;
use domain::ports::repository::solution::SolutionRepository;
use validator::Validate;

use crate::{
    error::SolutionError,
    model::solution::{create_comment::CreateCommentInput, output::CommentOutput},
};

#[derive(new)]
pub struct CreateCommentUsecase {
    repository: Arc<dyn SolutionRepository>,
}

impl CreateCommentUsecase {
    pub async fn run(&self, input: CreateCommentInput) -> Result<CommentOutput, SolutionError> {
        input
            .validate()
            .map_err(|e| SolutionError::BadRequest(e.to_string()))?;

        let exists = self.repository.solution_exists(input.solution_id).await?;
        if !exists {
            return Err(SolutionError::NotFound("solution not found".to_string()));
        }

        let comment = self
            .repository
            .create_comment(&input.user_id, input.solution_id, &input.body_md)
            .await?;

        Ok(CommentOutput::from(comment))
    }
}
