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
