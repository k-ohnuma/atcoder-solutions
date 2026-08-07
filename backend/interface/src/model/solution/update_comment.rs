use domain::model::{solution::comment_id::CommentId, user::id::UserId};
use serde::{Deserialize, Serialize};
use usecase::model::solution::{output::CommentOutput, update_comment::UpdateCommentInput};
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCommentRequest {
    pub body_md: String,
}

pub fn from_req_for_input(
    user_id: UserId,
    comment_id: CommentId,
    req: UpdateCommentRequest,
) -> UpdateCommentInput {
    UpdateCommentInput {
        user_id,
        comment_id,
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

impl From<CommentOutput> for UpdateCommentResponse {
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

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use domain::model::user::{id::UserId, name::UserName};
    use serde_json::json;
    use usecase::model::solution::output::CommentOutput;
    use uuid::Uuid;

    use super::{UpdateCommentRequest, UpdateCommentResponse, from_req_for_input};

    #[test]
    fn deserialize_update_comment_request_from_camel_case() {
        let comment_id = Uuid::now_v7();
        let raw = json!({
            "bodyMd": "updated"
        });
        let req: UpdateCommentRequest = serde_json::from_value(raw).expect("valid json");
        let input = from_req_for_input(
            UserId::try_new("uid".to_string()).expect("valid user ID"),
            comment_id.into(),
            req,
        );
        assert_eq!(input.user_id.as_str(), "uid");
        assert_eq!(input.comment_id.into_uuid(), comment_id);
        assert_eq!(input.body_md, "updated");
    }

    #[test]
    fn deserialize_update_comment_request_requires_body_md() {
        let raw = json!({});

        let result = serde_json::from_value::<UpdateCommentRequest>(raw);

        assert!(result.is_err());
    }

    #[test]
    fn update_comment_response_conversion_keeps_fields() {
        let now = Utc::now();
        let dto = CommentOutput {
            id: Uuid::now_v7().into(),
            user_id: UserId::try_new("uid".to_string()).expect("valid user ID"),
            user_name: UserName::try_new("alice".to_string()).expect("valid user name"),
            solution_id: Uuid::now_v7().into(),
            body_md: "body".to_string(),
            created_at: now,
            updated_at: now,
        };
        let resp = UpdateCommentResponse::from(dto);
        assert_eq!(resp.user_name, "alice");
        assert_eq!(resp.body_md, "body");
    }
}
