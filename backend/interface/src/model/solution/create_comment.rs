use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use usecase::dto::solution::CreatedCommentView;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommentRequest {
    pub solution_id: Uuid,
    pub body_md: String,
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

impl From<CreatedCommentView> for CreateCommentResponse {
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

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use serde_json::json;
    use uuid::Uuid;

    use usecase::dto::solution::CreatedCommentView;

    use super::{CreateCommentRequest, CreateCommentResponse};

    #[test]
    fn deserialize_create_comment_request_from_camel_case() {
        let solution_id = Uuid::now_v7();
        let raw = json!({
            "solutionId": solution_id,
            "bodyMd": "hello"
        });
        let req: CreateCommentRequest = serde_json::from_value(raw).expect("valid json");
        assert_eq!(req.solution_id, solution_id);
        assert_eq!(req.body_md, "hello");
    }

    #[test]
    fn create_comment_response_conversion_keeps_fields() {
        let now = Utc::now();
        let dto = CreatedCommentView {
            id: Uuid::now_v7(),
            user_id: "uid".to_string(),
            user_name: "alice".to_string(),
            solution_id: Uuid::now_v7(),
            body_md: "body".to_string(),
            created_at: now,
            updated_at: now,
        };

        let resp = CreateCommentResponse::from(dto);
        assert_eq!(resp.user_name, "alice");
        assert_eq!(resp.body_md, "body");
    }
}
