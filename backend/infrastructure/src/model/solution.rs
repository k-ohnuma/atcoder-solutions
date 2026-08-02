use chrono::{DateTime, Utc};
use domain::{
    error::problem::ProblemValidationError,
    model::{problem::id::ProblemId, solution::id::SolutionId},
    read_model::solution::{
        SolutionComment, SolutionDetails, SolutionListItem, UserSolutionListItem,
    },
};
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

impl TryFrom<SolutionListItemViewRaw> for SolutionListItem {
    type Error = ProblemValidationError;

    fn try_from(value: SolutionListItemViewRaw) -> Result<Self, Self::Error> {
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

        Ok(Self {
            id: SolutionId::new(id),
            title,
            problem_id: ProblemId::try_new(problem_id)?,
            user_id,
            user_name,
            votes_count,
            created_at,
            updated_at,
        })
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

impl TryFrom<UserSolutionListItemViewRaw> for UserSolutionListItem {
    type Error = ProblemValidationError;

    fn try_from(value: UserSolutionListItemViewRaw) -> Result<Self, Self::Error> {
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

        Ok(Self {
            id: SolutionId::new(id),
            title,
            problem_id: ProblemId::try_new(problem_id)?,
            problem_title,
            user_id,
            user_name,
            votes_count,
            created_at,
            updated_at,
        })
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
            solution_id: SolutionId::new(solution_id),
            body_md,
            created_at,
            updated_at,
        }
    }
}

pub struct SolutionDetailsViewRaw {
    pub id: Uuid,
    pub title: String,
    pub problem_id: String,
    pub contest_code: String,
    pub problem_title: String,
    pub user_id: String,
    pub user_name: String,
    pub tags: Vec<String>,
    pub body_md: String,
    pub submit_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<SolutionDetailsViewRaw> for SolutionDetails {
    type Error = ProblemValidationError;

    fn try_from(value: SolutionDetailsViewRaw) -> Result<Self, Self::Error> {
        Ok(Self {
            id: SolutionId::new(value.id),
            title: value.title,
            problem_id: ProblemId::try_new(value.problem_id)?,
            contest_code: value.contest_code,
            problem_title: value.problem_title,
            user_id: value.user_id,
            user_name: value.user_name,
            tags: value.tags,
            body_md: value.body_md,
            submit_url: value.submit_url,
            created_at: value.created_at,
            updated_at: value.updated_at,
        })
    }
}
