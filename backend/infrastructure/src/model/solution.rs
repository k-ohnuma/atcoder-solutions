use chrono::{DateTime, Utc};
use usecase::model::solution::{SolutionComment, SolutionListItem, UserSolutionListItem};
use uuid::Uuid;

pub struct SolutionListItemViewRaw {
    pub id: Uuid,
    pub title: String,
    pub problem_id: String,
    pub user_id: String,
    pub user_name: String,
    pub votes_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<SolutionListItemViewRaw> for SolutionListItem {
    fn from(value: SolutionListItemViewRaw) -> Self {
        let SolutionListItemViewRaw {
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

pub struct UserSolutionListItemViewRaw {
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

impl From<UserSolutionListItemViewRaw> for UserSolutionListItem {
    fn from(value: UserSolutionListItemViewRaw) -> Self {
        let UserSolutionListItemViewRaw {
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

pub struct SolutionCommentViewRaw {
    pub id: Uuid,
    pub user_id: String,
    pub user_name: String,
    pub solution_id: Uuid,
    pub body_md: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<SolutionCommentViewRaw> for SolutionComment {
    fn from(value: SolutionCommentViewRaw) -> Self {
        let SolutionCommentViewRaw {
            id,
            user_id,
            user_name,
            solution_id,
            body_md,
            created_at,
            updated_at,
        } = value;

        Self {
            id,
            user_id,
            user_name,
            solution_id,
            body_md,
            created_at,
            updated_at,
        }
    }
}
