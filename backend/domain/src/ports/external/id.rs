use uuid::Uuid;

pub trait IdProviderPort: Send + Sync {
    fn new_solution_id(&self) -> Uuid;
    fn new_tag_id(&self) -> Uuid;
}
