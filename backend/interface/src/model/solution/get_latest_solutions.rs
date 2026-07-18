use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use usecase::dto::solution::SolutionListItemView;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLatestSolutionsRequest {
    pub sort_by: Option<String>,
    pub limit: Option<i32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLatestSolutionsResponse {
    pub id: Uuid,
    pub title: String,
    pub problem_id: String,
    pub user_id: String,
    pub user_name: String,
    pub votes_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<SolutionListItemView> for GetLatestSolutionsResponse {
    fn from(value: SolutionListItemView) -> Self {
        let SolutionListItemView {
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
            id,
            title,
            problem_id,
            user_id,
            user_name,
            votes_count,
            created_at,
            updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::GetLatestSolutionsRequest;

    #[test]
    fn deserialize_latest_solutions_query_from_camel_case() {
        let raw = json!({
            "sortBy": "latest",
            "limit": 50
        });

        let req: GetLatestSolutionsRequest = serde_json::from_value(raw).expect("valid json");

        assert_eq!(req.sort_by.as_deref(), Some("latest"));
        assert_eq!(req.limit, Some(50));
    }

    #[test]
    fn deserialize_latest_solutions_query_allows_empty_conditions() {
        let raw = json!({});

        let req: GetLatestSolutionsRequest = serde_json::from_value(raw).expect("valid json");

        assert_eq!(req.sort_by, None);
        assert_eq!(req.limit, None);
    }
}
