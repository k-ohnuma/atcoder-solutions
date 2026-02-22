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

pub fn from_req_for_input(user_id: String, req: CreateSolutionRequest) -> CreateSolutionInput {
    CreateSolutionInput {
        user_id,
        title: req.title,
        problem_id: req.problem_id,
        body_md: req.body_md,
        submit_url: req.submit_url,
        tags: req.tags,
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSolutionResponse {
    solution_id: Uuid,
}

impl From<Uuid> for CreateSolutionResponse {
    fn from(value: Uuid) -> Self {
        Self { solution_id: value }
    }
}

#[cfg(test)]
mod tests {
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
        let input = from_req_for_input("uid".to_string(), req);
        assert_eq!(input.user_id, "uid");
        assert_eq!(input.problem_id, "abc100_a");
        assert_eq!(input.tags.len(), 2);
    }

    #[test]
    fn serialize_create_solution_response_as_camel_case() {
        let id = Uuid::now_v7();
        let resp = CreateSolutionResponse::from(id);
        let v = serde_json::to_value(resp).expect("serialize");
        let got = v
            .get("solutionId")
            .and_then(|e| e.as_str())
            .expect("solutionId");
        assert_eq!(got, id.to_string());
    }
}
