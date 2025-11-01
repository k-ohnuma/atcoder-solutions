use std::sync::Arc;

use derive_new::new;
use domain::{
    model::solution::Solution,
    ports::{external::id::IdProviderPort, repository::solution::tx::SolutionTxManager},
};
use itertools::Itertools;
use uuid::Uuid;

use super::SolutionError;

pub struct CreateSolutionInput {
    pub user_id: String,
    pub problem_id: String,
    pub body_md: String,
    pub submit_url: String,
    pub tags: Vec<String>,
}


pub fn from_create_solution_input_for_solution(
    uuid: Uuid,
    input: &CreateSolutionInput,
) -> Solution {
    Solution {
        id: uuid,
        user_id: input.user_id.to_owned(),
        problem_id: input.problem_id.to_owned(),
        body_md: input.body_md.to_owned(),
        submit_url: input.submit_url.to_owned(),
    }
}

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

        uow.tags().upsert(&tags).await?;
        uow.solutions().create(&solution).await?;

        uow.commit().await?;

        Ok(uid)
    }
}
