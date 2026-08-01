use uuid::Uuid;

pub mod tags;

pub struct Solution {
    pub id: Uuid,
    pub title: String,
    pub problem_id: String,
    pub user_id: String,
    pub body_md: String,
    pub submit_url: String,
}
