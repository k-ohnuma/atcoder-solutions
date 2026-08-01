use crate::model::solution::id::SolutionId;

pub trait IdProviderPort: Send + Sync {
    fn new_solution_id(&self) -> SolutionId;
}
