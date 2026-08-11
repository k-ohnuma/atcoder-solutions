use domain::model::solution::id::SolutionId;
use domain::model::user::id::UserId;
use serde::{Deserialize, Serialize};
use usecase::model::solution::create::CreateSolutionInput;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSolutionRequest {
    problem_id: String,
    title: String,
    body_md: String,
    submit_url: String,
    tags: Vec<String>,
}

pub fn from_req_for_input(
    user_id: UserId,
    req: CreateSolutionRequest,
) -> Result<CreateSolutionInput, usecase::error::SolutionError> {
    CreateSolutionInput::try_new(
        user_id,
        req.problem_id,
        req.title,
        req.body_md,
        req.submit_url,
        req.tags,
    )
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSolutionResponse {
    solution_id: Uuid,
}

impl From<SolutionId> for CreateSolutionResponse {
    fn from(value: SolutionId) -> Self {
        Self {
            solution_id: value.into_uuid(),
        }
    }
}

#[cfg(test)]
mod tests {
    use domain::model::user::id::UserId;
    use serde_json::json;
    use uuid::Uuid;

    use super::{CreateSolutionRequest, CreateSolutionResponse, from_req_for_input};

    #[test]
    fn deserialize_create_solution_request_from_camel_case() {
        let raw = json!({
            "problemId": "abc100_a",
            "title": "title",
            "bodyMd": "# body",
            "submitUrl": "https://example.com",
            "tags": ["dp", "graph"]
        });
        let req: CreateSolutionRequest = serde_json::from_value(raw).expect("valid json");
        let input = from_req_for_input(
            UserId::try_new("uid".to_string()).expect("valid user ID"),
            req,
        )
        .expect("valid input");
        assert_eq!(input.user_id.as_str(), "uid");
        assert_eq!(input.problem_id.as_str(), "abc100_a");
        assert_eq!(input.tags.len(), 2);
    }

    #[test]
    fn serialize_create_solution_response_as_camel_case() {
        let id = Uuid::now_v7();
        let resp = CreateSolutionResponse::from(domain::model::solution::id::SolutionId::new(id));
        let v = serde_json::to_value(resp).expect("serialize");
        let got = v
            .get("solutionId")
            .and_then(|e| e.as_str())
            .expect("solutionId");
        assert_eq!(got, id.to_string());
    }
}
