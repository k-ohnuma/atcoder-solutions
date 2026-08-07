use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use usecase::model::solution::output::UserSolutionListItemOutput;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSolutionsByUserNameRequest {
    pub sort_by: Option<String>,
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

impl From<UserSolutionListItemOutput> for GetSolutionsByUserNameResponse {
    fn from(value: UserSolutionListItemOutput) -> Self {
        let UserSolutionListItemOutput {
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
            id: id.into_uuid(),
            title: title.into_string(),
            problem_id: problem_id.into_string(),
            problem_title: problem_title.into_string(),
            user_id: user_id.into_string(),
            user_name: user_name.into_string(),
            votes_count,
            created_at,
            updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use domain::model::{
        solution::content::SolutionTitle,
        user::{id::UserId, name::UserName},
    };
    use serde_json::json;
    use usecase::model::solution::output::UserSolutionListItemOutput;
    use uuid::Uuid;

    use super::{GetSolutionsByUserNameRequest, GetSolutionsByUserNameResponse};

    #[test]
    fn deserialize_camel_case_sort_by() {
        let raw = json!({
            "sortBy": "votes"
        });
        let req: GetSolutionsByUserNameRequest = serde_json::from_value(raw).expect("valid json");
        assert_eq!(req.sort_by.as_deref(), Some("votes"));
    }

    #[test]
    fn response_conversion_keeps_problem_title() {
        let id = Uuid::now_v7();
        let now = Utc::now();
        let dto = UserSolutionListItemOutput {
            id: id.into(),
            title: SolutionTitle::try_new("title".to_string()).expect("valid solution title"),
            problem_id: domain::model::problem::id::ProblemId::try_new("abc100_a".to_string())
                .expect("valid problem ID"),
            problem_title: domain::model::problem::title::ProblemTitle::try_new(
                "A - Sample".to_string(),
            )
            .expect("valid problem title"),
            user_id: UserId::try_new("uid".to_string()).expect("valid user ID"),
            user_name: UserName::try_new("alice".to_string()).expect("valid user name"),
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
