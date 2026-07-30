use usecase::model::{
    contests::ContestError, problem::ProblemError, solution::SolutionError, user::UserError,
};

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
            ProblemError::DBError(reason) => {
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
            SolutionError::Conflict(reason) => HttpError::Conflict(reason),
            SolutionError::DBError(reason) => {
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
            UserError::Conflict(reason) => HttpError::Conflict(reason),
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
            UserError::DBError(reason) => {
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
            ContestError::NotFound(reason) => HttpError::NotFound(reason),
            ContestError::DBError(reason) => {
                tracing::error!(domain = "contest", error.message = %reason, "internal error");
                HttpError::Internal(reason)
            }
        }
    }
}
