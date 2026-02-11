use domain::error::{external::ExternalError, repository::RepositoryError};
use reqwest::StatusCode;
use sqlx::error::DatabaseError;

pub fn map_sqlx_error(err: sqlx::Error) -> RepositoryError {
    match err {
        sqlx::Error::RowNotFound => RepositoryError::NotFound(err.to_string()),
        sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed | sqlx::Error::Io(_) => {
            RepositoryError::Connection(err.to_string())
        }
        sqlx::Error::Database(db_err) => map_database_error(&*db_err),
        _ => RepositoryError::Query(err.to_string()),
    }
}

fn map_database_error(db_err: &dyn DatabaseError) -> RepositoryError {
    match db_err.code().as_deref() {
        Some("23505") => RepositoryError::UniqueViolation(db_err.message().to_string()),
        Some("23503") => RepositoryError::ForeignKeyViolation(db_err.message().to_string()),
        Some("23502") => RepositoryError::NotNullViolation(db_err.message().to_string()),
        Some("23514") => RepositoryError::CheckViolation(db_err.message().to_string()),
        _ => RepositoryError::Query(db_err.message().to_string()),
    }
}

pub fn map_reqwest_error(err: reqwest::Error) -> ExternalError {
    let url = err.url();
    tracing::error!(error = ?err, url = ?url, "external request failed");

    if err.is_timeout() || err.is_connect() {
        return ExternalError::ExternalUnavailable;
    }
    if let Some(s) = err.status() {
        return match s {
            StatusCode::NOT_FOUND => ExternalError::NotFound,
            StatusCode::UNAUTHORIZED => ExternalError::Unauthorized,
            StatusCode::FORBIDDEN => ExternalError::Forbidden,
            StatusCode::TOO_MANY_REQUESTS => ExternalError::RateLimited,
            s if s.is_server_error() => ExternalError::ExternalUnavailable,
            _ => ExternalError::Other(err.to_string()),
        };
    }
    ExternalError::Other(err.to_string())
}
