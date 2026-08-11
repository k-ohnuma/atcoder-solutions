use domain::error::repository::RepositoryError;
use sqlx::error::DatabaseError;

pub fn map_sqlx_error(err: sqlx::Error) -> RepositoryError {
    let kind = match &err {
        sqlx::Error::RowNotFound => "row_not_found",
        sqlx::Error::PoolTimedOut => "pool_timed_out",
        sqlx::Error::PoolClosed => "pool_closed",
        sqlx::Error::Io(_) => "io",
        sqlx::Error::Database(_) => "database",
        _ => "other",
    };
    tracing::error!(error.kind = kind, error.message = %err, error.debug = ?err, "sqlx error");

    match err {
        sqlx::Error::RowNotFound => RepositoryError::Unexpected(err.to_string()),
        sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed | sqlx::Error::Io(_) => {
            RepositoryError::Unavailable(err.to_string())
        }
        sqlx::Error::Database(db_err) => map_database_error(&*db_err),
        _ => RepositoryError::Unexpected(err.to_string()),
    }
}

fn map_database_error(db_err: &dyn DatabaseError) -> RepositoryError {
    tracing::error!(
        error.code = ?db_err.code().as_deref(),
        error.message = db_err.message(),
        "sqlx database error"
    );

    RepositoryError::Unexpected(db_err.message().to_string())
}
