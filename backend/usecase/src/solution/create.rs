use std::sync::Arc;

use derive_new::new;
use domain::model::solution::tags::Tags;
use domain::ports::{
    external::id::IdProviderPort,
    repository::{problem::ProblemRepository, solution::tx::SolutionTxManager},
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::SolutionError,
    model::solution::create::{CreateSolutionInput, from_create_solution_input_for_solution},
};

#[derive(new)]
pub struct CreateSolutionUsecase {
    idp: Arc<dyn IdProviderPort>,
    txm: Arc<dyn SolutionTxManager>,
    problem_repository: Arc<dyn ProblemRepository>,
}

impl CreateSolutionUsecase {
    pub async fn run(&self, input: CreateSolutionInput) -> Result<Uuid, SolutionError> {
        input
            .validate()
            .map_err(|e| SolutionError::BadRequest(e.to_string()))?;

        let problem_exists = self
            .problem_repository
            .problem_exists(&input.problem_id)
            .await?;
        if !problem_exists {
            return Err(SolutionError::BadRequest(
                "problem_id not found".to_string(),
            ));
        }

        let tags = Tags::try_new(input.tags.clone())?;

        let mut uow = self.txm.begin().await?;
        let uid = self.idp.new_solution_id();
        let solution = from_create_solution_input_for_solution(uid, &input);

        let tag_ids = uow.tags().upsert(&tags).await?;
        let solution_id = uow.solutions().create(&solution).await?;

        uow.solutions().replace_tags(solution_id, &tag_ids).await?;

        uow.commit().await?;

        Ok(uid)
    }
}
