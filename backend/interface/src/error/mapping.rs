use usecase::error::{ContestError, ProblemError, SolutionError, UserError};

use super::HttpError;

pub trait ToHttpError {
    fn to_http_error(self) -> HttpError;
}

pub trait IntoApiResult<T> {
    fn into_api_result(self) -> Result<T, HttpError>;
}

impl<T, E> IntoApiResult<T> for Result<T, E>
where
    E: ToHttpError,
{
    fn into_api_result(self) -> Result<T, HttpError> {
        self.map_err(|e| e.to_http_error())
    }
}

impl ToHttpError for ProblemError {
    fn to_http_error(self) -> HttpError {
        match self {
            ProblemError::BadRequest(reason) => HttpError::BadRequest(reason),
            ProblemError::NotFound(reason) => HttpError::NotFound(reason),
            ProblemError::Unavailable(reason) => HttpError::ServiceUnavailable(reason),
            ProblemError::Internal(reason) => {
                tracing::error!(domain = "problem", error.message = %reason, "internal error");
                HttpError::Internal(reason)
            }
        }
    }
}

impl ToHttpError for SolutionError {
    fn to_http_error(self) -> HttpError {
        match self {
            SolutionError::BadRequest(reason) => HttpError::BadRequest(reason),
            SolutionError::Forbidden(reason) => HttpError::Forbidden(reason),
            SolutionError::NotFound(reason) => HttpError::NotFound(reason),
            SolutionError::Unavailable(reason) => HttpError::ServiceUnavailable(reason),
            SolutionError::Internal(reason) => {
                tracing::error!(domain = "solution", error.message = %reason, "internal error");
                HttpError::Internal(reason)
            }
        }
    }
}

impl ToHttpError for UserError {
    fn to_http_error(self) -> HttpError {
        match self {
            UserError::BadRequest(reason) => HttpError::BadRequest(reason),
            UserError::NotFound(reason) => HttpError::NotFound(reason),
            UserError::Unavailable(reason) => HttpError::ServiceUnavailable(reason),
            UserError::EmailAlreadyExists(reason) => HttpError::ConflictWithCode {
                message: reason,
                error_code: "EMAIL_ALREADY_EXISTS",
            },
            UserError::UserNameAlreadyExists(reason) => HttpError::ConflictWithCode {
                message: reason,
                error_code: "USER_NAME_ALREADY_EXISTS",
            },
            UserError::RequiresRecentLogin(reason) => HttpError::UnauthorizedWithCode {
                message: reason,
                error_code: "REQUIRES_RECENT_LOGIN",
            },
            UserError::Internal(reason) => {
                tracing::error!(domain = "user", error.message = %reason, "internal error");
                HttpError::Internal(reason)
            }
        }
    }
}

impl ToHttpError for ContestError {
    fn to_http_error(self) -> HttpError {
        match self {
            ContestError::BadRequest(reason) => HttpError::BadRequest(reason),
            ContestError::Unavailable(reason) => HttpError::ServiceUnavailable(reason),
            ContestError::Internal(reason) => {
                tracing::error!(domain = "contest", error.message = %reason, "internal error");
                HttpError::Internal(reason)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use domain::error::repository::RepositoryError;
    use usecase::error::{ContestError, ProblemError, SolutionError, UserError};

    use super::ToHttpError;

    #[test]
    fn repository_unavailable_maps_to_service_unavailable() {
        let message = || "database unavailable".to_string();

        assert_eq!(
            ProblemError::from(RepositoryError::Unavailable(message()))
                .to_http_error()
                .status_code(),
            StatusCode::SERVICE_UNAVAILABLE
        );
        assert_eq!(
            SolutionError::from(RepositoryError::Unavailable(message()))
                .to_http_error()
                .status_code(),
            StatusCode::SERVICE_UNAVAILABLE
        );
        assert_eq!(
            UserError::from(RepositoryError::Unavailable(message()))
                .to_http_error()
                .status_code(),
            StatusCode::SERVICE_UNAVAILABLE
        );
        assert_eq!(
            ContestError::from(RepositoryError::Unavailable(message()))
                .to_http_error()
                .status_code(),
            StatusCode::SERVICE_UNAVAILABLE
        );
    }

    #[test]
    fn repository_unexpected_maps_to_internal_server_error() {
        let error = ProblemError::from(RepositoryError::Unexpected(
            "unexpected database error".to_string(),
        ));

        assert_eq!(
            error.to_http_error().status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }
}
