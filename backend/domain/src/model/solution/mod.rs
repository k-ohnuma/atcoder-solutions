use uuid::Uuid;

pub struct Solution {
    pub id: Uuid,
    pub problem_id: String,
    pub user_id: String,
    pub body_md: String,
    pub submit_url: String,
}
