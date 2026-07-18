use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCommentResponse {
    pub comment_id: Uuid,
}

impl From<Uuid> for DeleteCommentResponse {
    fn from(value: Uuid) -> Self {
        Self { comment_id: value }
    }
}
