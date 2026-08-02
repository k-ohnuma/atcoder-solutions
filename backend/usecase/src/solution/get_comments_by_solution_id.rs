use std::sync::Arc;

use derive_new::new;
use domain::{model::solution::id::SolutionId, ports::repository::solution::SolutionRepository};

use crate::{error::SolutionError, model::solution::output::CommentOutput};

#[derive(new)]
pub struct GetCommentsBySolutionIdUsecase {
    repository: Arc<dyn SolutionRepository>,
}

impl GetCommentsBySolutionIdUsecase {
    pub async fn run(&self, solution_id: SolutionId) -> Result<Vec<CommentOutput>, SolutionError> {
        let exists = self.repository.solution_exists(solution_id).await?;
        if !exists {
            return Err(SolutionError::NotFound("solution not found".to_string()));
        }
        let comments = self
            .repository
            .get_comments_by_solution_id(solution_id)
            .await?;
        Ok(comments.into_iter().map(CommentOutput::from).collect())
    }
}
