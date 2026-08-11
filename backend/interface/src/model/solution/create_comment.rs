use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use usecase::model::solution::output::CommentOutput;
use usecase::{error::SolutionError, model::solution::create_comment::CreateCommentInput};
use uuid::Uuid;

use domain::model::{solution::id::SolutionId, user::id::UserId};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommentRequest {
    pub body_md: String,
}

pub fn from_req_for_input(
    user_id: UserId,
    solution_id: SolutionId,
    req: CreateCommentRequest,
) -> Result<CreateCommentInput, SolutionError> {
    CreateCommentInput::try_new(user_id, solution_id, req.body_md)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommentResponse {
    pub id: Uuid,
    pub user_id: String,
    pub user_name: String,
    pub solution_id: Uuid,
    pub body_md: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<CommentOutput> for CreateCommentResponse {
    fn from(value: CommentOutput) -> Self {
        Self {
            id: value.id.into_uuid(),
            user_id: value.user_id.into_string(),
            user_name: value.user_name.into_string(),
            solution_id: value.solution_id.into_uuid(),
            body_md: value.body.into_string(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use domain::model::{
        solution::comment_body::CommentBody,
        user::{id::UserId, name::UserName},
    };
    use serde_json::json;
    use uuid::Uuid;

    use usecase::model::solution::output::CommentOutput;

    use super::{CreateCommentRequest, CreateCommentResponse};

    #[test]
    fn deserialize_create_comment_request_from_camel_case() {
        let raw = json!({
            "bodyMd": "hello"
        });
        let req: CreateCommentRequest = serde_json::from_value(raw).expect("valid json");
        assert_eq!(req.body_md, "hello");
    }

    #[test]
    fn deserialize_create_comment_request_requires_body_md() {
        let raw = json!({});

        let result = serde_json::from_value::<CreateCommentRequest>(raw);

        assert!(result.is_err());
    }

    #[test]
    fn create_comment_response_conversion_keeps_fields() {
        let now = Utc::now();
        let dto = CommentOutput {
            id: Uuid::now_v7().into(),
            user_id: UserId::try_new("uid".to_string()).expect("valid user ID"),
            user_name: UserName::try_new("alice".to_string()).expect("valid user name"),
            solution_id: Uuid::now_v7().into(),
            body: CommentBody::try_new("body".to_string()).expect("valid comment body"),
            created_at: now,
            updated_at: now,
        };

        let resp = CreateCommentResponse::from(dto);
        assert_eq!(resp.user_name, "alice");
        assert_eq!(resp.body_md, "body");
    }
}
