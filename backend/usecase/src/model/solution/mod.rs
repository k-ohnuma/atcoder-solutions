pub mod create;

use chrono::{DateTime, Utc};
use domain::error::repository::RepositoryError;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolutionListSort {
    Latest,
    Votes,
}

#[derive(Debug, Clone)]
pub struct SolutionListItem {
    pub id: Uuid,
    pub title: String,
    pub problem_id: String,
    pub user_id: String,
    pub user_name: String,
    pub votes_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct SolutionDetails {
    pub id: Uuid,
    pub title: String,
    pub problem_id: String,
    pub problem_title: String,
    pub user_id: String,
    pub user_name: String,
    pub tags: Vec<String>,
    pub body_md: String,
    pub submit_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CreatedComment {
    pub id: Uuid,
    pub user_id: String,
    pub user_name: String,
    pub solution_id: Uuid,
    pub body_md: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct SolutionComment {
    pub id: Uuid,
    pub user_id: String,
    pub user_name: String,
    pub solution_id: Uuid,
    pub body_md: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Error)]
pub enum SolutionError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    DBError(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Conflict(String),
}

impl From<RepositoryError> for SolutionError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::NotFound(msg) => SolutionError::NotFound(msg),
            RepositoryError::TransactionError(msg) => SolutionError::DBError(msg.to_string()),
            RepositoryError::UniqueViolation(msg) => SolutionError::Conflict(msg),
            RepositoryError::ForeignKeyViolation(msg) => SolutionError::DBError(msg.to_string()),
            RepositoryError::NotNullViolation(msg) => SolutionError::DBError(msg.to_string()),
            RepositoryError::CheckViolation(msg) => SolutionError::DBError(msg.to_string()),
            RepositoryError::Connection(msg) => SolutionError::DBError(msg.to_string()),
            RepositoryError::Query(msg) => SolutionError::DBError(msg.to_string()),
            RepositoryError::Unexpected(msg) => SolutionError::DBError(msg.to_string()),
        }
    }
}
