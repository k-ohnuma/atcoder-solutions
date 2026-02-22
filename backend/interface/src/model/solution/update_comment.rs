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

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use serde_json::json;
    use usecase::dto::solution::CreatedCommentView;
    use uuid::Uuid;

    use super::{UpdateCommentRequest, UpdateCommentResponse, from_req_for_input};

    #[test]
    fn deserialize_update_comment_request_from_camel_case() {
        let comment_id = Uuid::now_v7();
        let raw = json!({
            "commentId": comment_id,
            "bodyMd": "updated"
        });
        let req: UpdateCommentRequest = serde_json::from_value(raw).expect("valid json");
        let input = from_req_for_input("uid".to_string(), req);
        assert_eq!(input.user_id, "uid");
        assert_eq!(input.comment_id, comment_id);
        assert_eq!(input.body_md, "updated");
    }

    #[test]
    fn update_comment_response_conversion_keeps_fields() {
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
        let resp = UpdateCommentResponse::from(dto);
        assert_eq!(resp.user_name, "alice");
        assert_eq!(resp.body_md, "body");
    }
}
