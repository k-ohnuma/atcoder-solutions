use serde::{Deserialize, Serialize};
use usecase::{
    dto::solution::CreatedCommentView, model::solution::update_comment::UpdateCommentInput,
};
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCommentRequest {
    pub comment_id: Uuid,
    pub body_md: String,
}

pub fn from_req_for_input(user_id: String, req: UpdateCommentRequest) -> UpdateCommentInput {
    UpdateCommentInput {
        user_id,
        comment_id: req.comment_id,
        body_md: req.body_md,
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCommentResponse {
    pub id: Uuid,
    pub user_id: String,
    pub user_name: String,
    pub solution_id: Uuid,
    pub body_md: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<CreatedCommentView> for UpdateCommentResponse {
    fn from(value: CreatedCommentView) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            user_name: value.user_name,
            solution_id: value.solution_id,
            body_md: value.body_md,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
