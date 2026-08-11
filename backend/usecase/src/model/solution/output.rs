use chrono::{DateTime, Utc};
use domain::model::{
    problem::{contest_code::ContestCode, id::ProblemId, title::ProblemTitle},
    solution::{
        comment_body::CommentBody,
        comment_id::CommentId,
        content::{SolutionBody, SolutionTitle, SubmitUrl},
        id::SolutionId,
        tags::Tags,
    },
    user::{id::UserId, name::UserName},
};
use domain::read_model::solution::{
    SolutionComment, SolutionDetails, SolutionListItem, UserSolutionListItem,
};

pub struct SolutionListItemOutput {
    pub id: SolutionId,
    pub title: SolutionTitle,
    pub problem_id: ProblemId,
    pub user_id: UserId,
    pub user_name: UserName,
    pub votes_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct SolutionOutput {
    pub id: SolutionId,
    pub title: SolutionTitle,
    pub problem_id: ProblemId,
    pub contest_code: ContestCode,
    pub problem_title: ProblemTitle,
    pub user_id: UserId,
    pub user_name: UserName,
    pub tags: Tags,
    pub body: SolutionBody,
    pub submit_url: SubmitUrl,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct UserSolutionListItemOutput {
    pub id: SolutionId,
    pub title: SolutionTitle,
    pub problem_id: ProblemId,
    pub problem_title: ProblemTitle,
    pub user_id: UserId,
    pub user_name: UserName,
    pub votes_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct CommentOutput {
    pub id: CommentId,
    pub user_id: UserId,
    pub user_name: UserName,
    pub solution_id: SolutionId,
    pub body: CommentBody,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<SolutionListItem> for SolutionListItemOutput {
    fn from(value: SolutionListItem) -> Self {
        Self {
            id: value.id,
            title: value.title,
            problem_id: value.problem_id,
            user_id: value.user_id,
            user_name: value.user_name,
            votes_count: value.votes_count,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<SolutionDetails> for SolutionOutput {
    fn from(value: SolutionDetails) -> Self {
        Self {
            id: value.id,
            title: value.title,
            problem_id: value.problem_id,
            contest_code: value.contest_code,
            problem_title: value.problem_title,
            user_id: value.user_id,
            user_name: value.user_name,
            tags: value.tags,
            body: value.body,
            submit_url: value.submit_url,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<UserSolutionListItem> for UserSolutionListItemOutput {
    fn from(value: UserSolutionListItem) -> Self {
        Self {
            id: value.id,
            title: value.title,
            problem_id: value.problem_id,
            problem_title: value.problem_title,
            user_id: value.user_id,
            user_name: value.user_name,
            votes_count: value.votes_count,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<SolutionComment> for CommentOutput {
    fn from(value: SolutionComment) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            user_name: value.user_name,
            solution_id: value.solution_id,
            body: value.body,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
