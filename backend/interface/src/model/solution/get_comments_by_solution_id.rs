use chrono::{DateTime, Utc};
use serde::Serialize;
use usecase::model::solution::output::CommentOutput;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCommentsBySolutionIdResponse {
    pub id: Uuid,
    pub user_id: String,
    pub user_name: String,
    pub solution_id: Uuid,
    pub body_md: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<CommentOutput> for GetCommentsBySolutionIdResponse {
    fn from(value: CommentOutput) -> Self {
        Self {
            id: value.id.into_uuid(),
            user_id: value.user_id.into_string(),
            user_name: value.user_name.into_string(),
            solution_id: value.solution_id.into_uuid(),
            body_md: value.body_md,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
