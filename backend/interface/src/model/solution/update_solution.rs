use serde::{Deserialize, Serialize};
use usecase::model::solution::update::UpdateSolutionInput;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSolutionRequest {
    pub solution_id: Uuid,
    pub title: String,
    pub body_md: String,
    pub submit_url: String,
    pub tags: Vec<String>,
}

pub fn from_req_for_input(user_id: String, req: UpdateSolutionRequest) -> UpdateSolutionInput {
    UpdateSolutionInput {
        user_id,
        solution_id: req.solution_id,
        title: req.title,
        body_md: req.body_md,
        submit_url: req.submit_url,
        tags: req.tags,
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSolutionResponse {
    pub solution_id: Uuid,
}

impl From<Uuid> for UpdateSolutionResponse {
    fn from(value: Uuid) -> Self {
        Self { solution_id: value }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use uuid::Uuid;

    use super::{UpdateSolutionRequest, UpdateSolutionResponse, from_req_for_input};

    #[test]
    fn deserialize_update_solution_request_from_camel_case() {
        let solution_id = Uuid::now_v7();
        let raw = json!({
            "solutionId": solution_id,
            "title": "new title",
            "bodyMd": "new body",
            "submitUrl": "",
            "tags": ["dp"]
        });
        let req: UpdateSolutionRequest = serde_json::from_value(raw).expect("valid json");
        let input = from_req_for_input("uid".to_string(), req);
        assert_eq!(input.user_id, "uid");
        assert_eq!(input.solution_id, solution_id);
        assert_eq!(input.title, "new title");
    }

    #[test]
    fn serialize_update_solution_response_as_camel_case() {
        let solution_id = Uuid::now_v7();
        let resp = UpdateSolutionResponse::from(solution_id);
        let v = serde_json::to_value(resp).expect("serialize");
        let got = v
            .get("solutionId")
            .and_then(|e| e.as_str())
            .expect("solutionId");
        assert_eq!(got, solution_id.to_string());
    }
}
