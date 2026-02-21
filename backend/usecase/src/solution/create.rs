use std::sync::Arc;

use derive_new::new;
use domain::ports::{external::id::IdProviderPort, repository::solution::tx::SolutionTxManager};
use itertools::Itertools;
use uuid::Uuid;
use validator::Validate;

use crate::model::solution::{
    SolutionError,
    create::{CreateSolutionInput, from_create_solution_input_for_solution},
};
use crate::service::solution::SolutionService;

#[derive(new)]
pub struct CreateSolutionUsecase {
    idp: Arc<dyn IdProviderPort>,
    txm: Arc<dyn SolutionTxManager>,
    service: Arc<dyn SolutionService>,
}

impl CreateSolutionUsecase {
    pub async fn run(&self, input: CreateSolutionInput) -> Result<Uuid, SolutionError> {
        input
            .validate()
            .map_err(|e| SolutionError::BadRequest(e.to_string()))?;

        let problem_exists = self.service.problem_exists(&input.problem_id).await?;
        if !problem_exists {
            return Err(SolutionError::BadRequest(
                "problem_id not found".to_string(),
            ));
        }

        let mut uow = self.txm.begin().await?;
        let uid = self.idp.new_solution_id();
        let solution = from_create_solution_input_for_solution(uid, &input);

        let tags = input
            .tags
            .into_iter()
            .filter(|e| !e.trim().is_empty())
            .sorted()
            .dedup()
            .collect_vec();
        if tags.len() > 6 {
            return Err(SolutionError::BadRequest(
                "tags must be 6 or fewer".to_string(),
            ));
        }
        if tags.iter().any(|tag| tag.chars().count() > 24) {
            return Err(SolutionError::BadRequest("tag is too long".to_string()));
        }

        let tag_ids = uow.tags().upsert(&tags).await?;
        let solution_id = uow.solutions().create(&solution).await?;

        uow.solutions().replace_tags(solution_id, &tag_ids).await?;

        uow.commit().await?;

        Ok(uid)
    }
}
