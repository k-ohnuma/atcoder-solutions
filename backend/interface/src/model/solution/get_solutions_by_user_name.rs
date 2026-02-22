use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use usecase::{dto::solution::UserSolutionListItemView, model::solution::SolutionListSort};
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSolutionsByUserNameRequest {
    pub user_name: String,
    pub sort_by: Option<String>,
}

impl GetSolutionsByUserNameRequest {
    pub fn list_sort(&self) -> SolutionListSort {
        match self.sort_by.as_deref() {
            Some("votes") => SolutionListSort::Votes,
            _ => SolutionListSort::Latest,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSolutionsByUserNameResponse {
    pub id: Uuid,
    pub title: String,
    pub problem_id: String,
    pub problem_title: String,
    pub user_id: String,
    pub user_name: String,
    pub votes_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<UserSolutionListItemView> for GetSolutionsByUserNameResponse {
    fn from(value: UserSolutionListItemView) -> Self {
        let UserSolutionListItemView {
            id,
            title,
            problem_id,
            problem_title,
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
            problem_title,
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
    use usecase::{dto::solution::UserSolutionListItemView, model::solution::SolutionListSort};
    use uuid::Uuid;

    use super::{GetSolutionsByUserNameRequest, GetSolutionsByUserNameResponse};

    #[test]
    fn list_sort_defaults_to_latest() {
        let req = GetSolutionsByUserNameRequest {
            user_name: "alice".to_string(),
            sort_by: None,
        };
        assert!(matches!(req.list_sort(), SolutionListSort::Latest));
    }

    #[test]
    fn list_sort_votes_is_mapped() {
        let req = GetSolutionsByUserNameRequest {
            user_name: "alice".to_string(),
            sort_by: Some("votes".to_string()),
        };
        assert!(matches!(req.list_sort(), SolutionListSort::Votes));
    }

    #[test]
    fn deserialize_camel_case_sort_by() {
        let raw = json!({
            "userName": "alice",
            "sortBy": "votes"
        });
        let req: GetSolutionsByUserNameRequest = serde_json::from_value(raw).expect("valid json");
        assert_eq!(req.user_name, "alice");
        assert_eq!(req.sort_by.as_deref(), Some("votes"));
    }

    #[test]
    fn response_conversion_keeps_problem_title() {
        let id = Uuid::now_v7();
        let now = Utc::now();
        let dto = UserSolutionListItemView {
            id,
            title: "title".to_string(),
            problem_id: "abc100_a".to_string(),
            problem_title: "A - Sample".to_string(),
            user_id: "uid".to_string(),
            user_name: "alice".to_string(),
            votes_count: 7,
            created_at: now,
            updated_at: now,
        };

        let resp = GetSolutionsByUserNameResponse::from(dto);
        assert_eq!(resp.id, id);
        assert_eq!(resp.problem_title, "A - Sample");
        assert_eq!(resp.votes_count, 7);
    }
}
