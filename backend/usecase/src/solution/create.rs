use std::sync::Arc;

use derive_new::new;
use domain::model::solution::{
    Solution,
    content::{SolutionBody, SolutionTitle, SubmitUrl},
    tags::Tags,
};
use domain::ports::{
    external::id::IdProviderPort,
    repository::{problem::ProblemRepository, solution::tx::SolutionTxManager},
};

use crate::{error::SolutionError, model::solution::create::CreateSolutionInput};

#[derive(new)]
pub struct CreateSolutionUsecase {
    idp: Arc<dyn IdProviderPort>,
    txm: Arc<dyn SolutionTxManager>,
    problem_repository: Arc<dyn ProblemRepository>,
}

impl CreateSolutionUsecase {
    pub async fn run(
        &self,
        input: CreateSolutionInput,
    ) -> Result<domain::model::solution::id::SolutionId, SolutionError> {
        let problem_exists = self
            .problem_repository
            .problem_exists(&input.problem_id)
            .await?;
        if !problem_exists {
            return Err(SolutionError::BadRequest(
                "problem_id not found".to_string(),
            ));
        }

        let tags = Tags::try_new(input.tags)?;
        let title = SolutionTitle::try_new(input.title)?;
        let body = SolutionBody::try_new(input.body_md)?;
        let submit_url = SubmitUrl::try_new(input.submit_url)?;

        let mut uow = self.txm.begin().await?;
        let solution_id = self.idp.new_solution_id();
        let solution = Solution::new(
            solution_id,
            input.problem_id,
            input.user_id,
            title,
            body,
            submit_url,
        );

        let tag_ids = uow.tags().upsert(&tags).await?;
        let persisted_solution_id = uow.solutions().create(&solution).await?;

        uow.solutions()
            .replace_tags(persisted_solution_id, &tag_ids)
            .await?;

        uow.commit().await?;

        Ok(solution_id)
    }
}
