use domain::model::solution::comment_id::CommentId;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCommentResponse {
    pub comment_id: Uuid,
}

impl From<CommentId> for DeleteCommentResponse {
    fn from(value: CommentId) -> Self {
        Self {
            comment_id: value.into_uuid(),
        }
    }
}
