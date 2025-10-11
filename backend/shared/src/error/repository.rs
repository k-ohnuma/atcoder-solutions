use sqlx::error::DatabaseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Failed to execute transaction.")]
    TransactionError(#[source] sqlx::Error),

    #[error("Record not found.")]
    NotFound,

    #[error("Unique constraint violation: {0}")]
    UniqueViolation(String),

    #[error("Foreign key constraint violation: {0}")]
    ForeignKeyViolation(String),

    #[error("NOT NULL constraint violation: {0}")]
    NotNullViolation(String),

    #[error("CHECK constraint violation: {0}")]
    CheckViolation(String),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Query error: {0}")]
    Query(String),

    #[error("Unexpected database error: {0}")]
    Unexpected(String),
}

impl From<sqlx::Error> for RepositoryError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => RepositoryError::NotFound,
            sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed | sqlx::Error::Io(_) => {
                RepositoryError::Connection(err.to_string())
            }
            sqlx::Error::Database(db_err) => map_database_error(&*db_err),
            _ => RepositoryError::Query(err.to_string()),
        }
    }
}

fn map_database_error(db_err: &dyn DatabaseError) -> RepositoryError {
    // NOTE: https://www.postgresql.jp/document/15/html/errcodes-appendix.html
    match db_err.code().as_deref() {
        Some("23505") => RepositoryError::UniqueViolation(db_err.message().to_string()),
        Some("23503") => RepositoryError::ForeignKeyViolation(db_err.message().to_string()),
        Some("23502") => RepositoryError::NotNullViolation(db_err.message().to_string()),
        Some("23514") => RepositoryError::CheckViolation(db_err.message().to_string()),
        _ => RepositoryError::Query(db_err.message().to_string()),
    }
}
