use derive_new::new;
use domain::ports::external::id::IdProviderPort;
use uuid::Uuid;

#[derive(new)]
pub struct UuidProvider;

impl IdProviderPort for UuidProvider {
    fn new_solution_id(&self) -> Uuid {
        Uuid::now_v7()
    }
}
