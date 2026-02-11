use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Failed to execute transaction: {0}")]
    TransactionError(String),

    #[error("Record not found: {0}")]
    NotFound(String),

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
