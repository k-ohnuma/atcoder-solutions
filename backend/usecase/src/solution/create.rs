use std::sync::Arc;

use derive_new::new;
use domain::ports::{external::id::IdProviderPort, repository::solution::tx::SolutionTxManager};
use itertools::Itertools;
use uuid::Uuid;

use crate::model::solution::{
    SolutionError,
    create::{CreateSolutionInput, from_create_solution_input_for_solution},
};

#[derive(new)]
pub struct CreateSolutionUsecase {
    idp: Arc<dyn IdProviderPort>,
    txm: Arc<dyn SolutionTxManager>,
}

impl CreateSolutionUsecase {
    pub async fn run(&self, input: CreateSolutionInput) -> Result<Uuid, SolutionError> {
        let mut uow = self.txm.begin().await?;
        let uid = self.idp.new_solution_id();
        let solution = from_create_solution_input_for_solution(uid, &input);

        let tags = input
            .tags
            .to_owned()
            .into_iter()
            .filter(|e| !e.trim().is_empty())
            .sorted()
            .dedup()
            .collect_vec();

        let tag_ids = uow.tags().upsert(&tags).await?;
        let solution_id = uow.solutions().create(&solution).await?;

        uow.solutions().replace_tags(solution_id, &tag_ids).await?;

        uow.commit().await?;

        Ok(uid)
    }
}
