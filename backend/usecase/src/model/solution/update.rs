use domain::model::{solution::id::SolutionId, user::id::UserId};

pub struct UpdateSolutionInput {
    pub user_id: UserId,
    pub solution_id: SolutionId,
    pub title: String,
    pub body_md: String,
    pub submit_url: String,
    pub tags: Vec<String>,
}
