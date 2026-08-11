use chrono::{DateTime, Utc};
use domain::{
    error::repository::RepositoryError,
    model::{
        problem::{contest_code::ContestCode, id::ProblemId, title::ProblemTitle},
        solution::{
            comment_body::CommentBody,
            comment_id::CommentId,
            content::{SolutionBody, SolutionTitle, SubmitUrl},
            id::SolutionId,
            tags::Tags,
        },
        user::{id::UserId, name::UserName},
    },
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
    type Error = RepositoryError;

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
            title: SolutionTitle::try_new(title)
                .map_err(|error| RepositoryError::Unexpected(error.to_string()))?,
            problem_id: ProblemId::try_new(problem_id)
                .map_err(|error| RepositoryError::Unexpected(error.to_string()))?,
            user_id: UserId::try_new(user_id)
                .map_err(|error| RepositoryError::Unexpected(error.to_string()))?,
            user_name: UserName::try_new(user_name)
                .map_err(|error| RepositoryError::Unexpected(error.to_string()))?,
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
    type Error = RepositoryError;

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
            title: SolutionTitle::try_new(title)
                .map_err(|error| RepositoryError::Unexpected(error.to_string()))?,
            problem_id: ProblemId::try_new(problem_id)
                .map_err(|error| RepositoryError::Unexpected(error.to_string()))?,
            problem_title: ProblemTitle::try_new(problem_title)
                .map_err(|error| RepositoryError::Unexpected(error.to_string()))?,
            user_id: UserId::try_new(user_id)
                .map_err(|error| RepositoryError::Unexpected(error.to_string()))?,
            user_name: UserName::try_new(user_name)
                .map_err(|error| RepositoryError::Unexpected(error.to_string()))?,
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

impl TryFrom<SolutionCommentViewRaw> for SolutionComment {
    type Error = RepositoryError;

    fn try_from(value: SolutionCommentViewRaw) -> Result<Self, Self::Error> {
        let SolutionCommentViewRaw {
            id,
            user_id,
            user_name,
            solution_id,
            body_md,
            created_at,
            updated_at,
        } = value;

        Ok(Self {
            id: CommentId::new(id),
            user_id: UserId::try_new(user_id)
                .map_err(|error| RepositoryError::Unexpected(error.to_string()))?,
            user_name: UserName::try_new(user_name)
                .map_err(|error| RepositoryError::Unexpected(error.to_string()))?,
            solution_id: SolutionId::new(solution_id),
            body: CommentBody::try_new(body_md)
                .map_err(|error| RepositoryError::Unexpected(error.to_string()))?,
            created_at,
            updated_at,
        })
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
    type Error = RepositoryError;

    fn try_from(value: SolutionDetailsViewRaw) -> Result<Self, Self::Error> {
        Ok(Self {
            id: SolutionId::new(value.id),
            title: SolutionTitle::try_new(value.title)
                .map_err(|error| RepositoryError::Unexpected(error.to_string()))?,
            problem_id: ProblemId::try_new(value.problem_id)
                .map_err(|error| RepositoryError::Unexpected(error.to_string()))?,
            contest_code: ContestCode::try_new(value.contest_code)
                .map_err(|error| RepositoryError::Unexpected(error.to_string()))?,
            problem_title: ProblemTitle::try_new(value.problem_title)
                .map_err(|error| RepositoryError::Unexpected(error.to_string()))?,
            user_id: UserId::try_new(value.user_id)
                .map_err(|error| RepositoryError::Unexpected(error.to_string()))?,
            user_name: UserName::try_new(value.user_name)
                .map_err(|error| RepositoryError::Unexpected(error.to_string()))?,
            tags: Tags::try_new(value.tags)
                .map_err(|error| RepositoryError::Unexpected(error.to_string()))?,
            body: SolutionBody::try_new(value.body_md)
                .map_err(|error| RepositoryError::Unexpected(error.to_string()))?,
            submit_url: SubmitUrl::try_new(value.submit_url)
                .map_err(|error| RepositoryError::Unexpected(error.to_string()))?,
            created_at: value.created_at,
            updated_at: value.updated_at,
        })
    }
}
