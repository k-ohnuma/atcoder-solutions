use derive_new::new;
use domain::model::solution::id::SolutionId;
use domain::ports::external::id::IdProviderPort;
use uuid::Uuid;

#[derive(new)]
pub struct UuidProvider;

impl IdProviderPort for UuidProvider {
    fn new_solution_id(&self) -> SolutionId {
        SolutionId::new(Uuid::now_v7())
    }
}
