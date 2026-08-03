use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use usecase::model::solution::output::SolutionListItemOutput;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSolutionsByProblemIdRequest {
    pub sort_by: Option<String>,
    pub limit: Option<i32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSolutionsByProblemIdResponse {
    pub id: Uuid,
    pub title: String,
    pub problem_id: String,
    pub user_id: String,
    pub user_name: String,
    pub votes_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<SolutionListItemOutput> for GetSolutionsByProblemIdResponse {
    fn from(value: SolutionListItemOutput) -> Self {
        let SolutionListItemOutput {
            id,
            title,
            problem_id,
            user_id,
            user_name,
            votes_count,
            created_at,
            updated_at,
        } = value;

        Self {
            id: id.into_uuid(),
            title,
            problem_id: problem_id.into_string(),
            user_id: user_id.into_string(),
            user_name,
            votes_count,
            created_at,
            updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use domain::model::user::id::UserId;
    use serde_json::json;
    use usecase::model::solution::output::SolutionListItemOutput;
    use uuid::Uuid;

    use super::{GetSolutionsByProblemIdRequest, GetSolutionsByProblemIdResponse};

    #[test]
    fn deserialize_camel_case_sort_by() {
        let raw = json!({
            "sortBy": "votes",
            "limit": 50
        });
        let req: GetSolutionsByProblemIdRequest = serde_json::from_value(raw).expect("valid json");
        assert_eq!(req.sort_by.as_deref(), Some("votes"));
        assert_eq!(req.limit, Some(50));
    }

    #[test]
    fn response_conversion_keeps_votes_count() {
        let id = Uuid::now_v7();
        let now = Utc::now();
        let dto = SolutionListItemOutput {
            id: id.into(),
            title: "title".to_string(),
            problem_id: domain::model::problem::id::ProblemId::try_new("abc100_a".to_string())
                .expect("valid problem ID"),
            user_id: UserId::try_new("uid".to_string()).expect("valid user ID"),
            user_name: "name".to_string(),
            votes_count: 42,
            created_at: now,
            updated_at: now,
        };

        let resp = GetSolutionsByProblemIdResponse::from(dto);
        assert_eq!(resp.id, id);
        assert_eq!(resp.votes_count, 42);
        assert_eq!(resp.user_name, "name");
    }
}
