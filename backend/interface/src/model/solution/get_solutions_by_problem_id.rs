use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use usecase::dto::solution::SolutionListItemView;
use usecase::model::solution::SolutionListSort;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSolutionsByProblemIdRequest {
    pub problem_id: String,
    pub sort_by: Option<String>,
}

impl GetSolutionsByProblemIdRequest {
    pub fn list_sort(&self) -> SolutionListSort {
        match self.sort_by.as_deref() {
            Some("votes") => SolutionListSort::Votes,
            _ => SolutionListSort::Latest,
        }
    }
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

impl From<SolutionListItemView> for GetSolutionsByProblemIdResponse {
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
    use chrono::Utc;
    use serde_json::json;
    use usecase::{dto::solution::SolutionListItemView, model::solution::SolutionListSort};
    use uuid::Uuid;

    use super::{GetSolutionsByProblemIdRequest, GetSolutionsByProblemIdResponse};

    #[test]
    fn list_sort_defaults_to_latest() {
        let req = GetSolutionsByProblemIdRequest {
            problem_id: "abc100_a".to_string(),
            sort_by: None,
        };
        assert!(matches!(req.list_sort(), SolutionListSort::Latest));
    }

    #[test]
    fn list_sort_votes_is_mapped() {
        let req = GetSolutionsByProblemIdRequest {
            problem_id: "abc100_a".to_string(),
            sort_by: Some("votes".to_string()),
        };
        assert!(matches!(req.list_sort(), SolutionListSort::Votes));
    }

    #[test]
    fn deserialize_camel_case_sort_by() {
        let raw = json!({
            "problemId": "abc100_a",
            "sortBy": "votes"
        });
        let req: GetSolutionsByProblemIdRequest = serde_json::from_value(raw).expect("valid json");
        assert_eq!(req.problem_id, "abc100_a");
        assert_eq!(req.sort_by.as_deref(), Some("votes"));
    }

    #[test]
    fn response_conversion_keeps_votes_count() {
        let id = Uuid::now_v7();
        let now = Utc::now();
        let dto = SolutionListItemView {
            id,
            title: "title".to_string(),
            problem_id: "abc100_a".to_string(),
            user_id: "uid".to_string(),
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
