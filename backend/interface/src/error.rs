use shared::error::http::HttpError;
use usecase::model::{
    contests::ContestError, problem::ProblemError, solution::SolutionError, user::UserError,
};

pub trait ToHttpError {
    fn to_http_error(self) -> HttpError;
}

impl ToHttpError for ProblemError {
    fn to_http_error(self) -> HttpError {
        match self {
            ProblemError::BadRequest(reason) => HttpError::BadRequest(reason),
            ProblemError::NotFound(reason) => HttpError::NotFound(reason),
            ProblemError::DBError(reason) => HttpError::Internal(reason),
        }
    }
}

impl ToHttpError for SolutionError {
    fn to_http_error(self) -> HttpError {
        match self {
            SolutionError::BadRequest(reason) => HttpError::BadRequest(reason),
            SolutionError::NotFound(reason) => HttpError::NotFound(reason),
            SolutionError::Conflict(reason) => HttpError::Conflict(reason),
            SolutionError::DBError(reason) => HttpError::Internal(reason),
        }
    }
}

impl ToHttpError for UserError {
    fn to_http_error(self) -> HttpError {
        match self {
            UserError::BadRequest(reason) => HttpError::BadRequest(reason),
            UserError::NotFound(reason) => HttpError::NotFound(reason),
            UserError::Conflict(reason) => HttpError::Conflict(reason),
            UserError::DBError(reason) => HttpError::Internal(reason),
        }
    }
}

impl ToHttpError for ContestError {
    fn to_http_error(self) -> HttpError {
        match self {
            ContestError::BadRequest(reason) => HttpError::BadRequest(reason),
            ContestError::NotFound(reason) => HttpError::NotFound(reason),
            ContestError::DBError(reason) => HttpError::Internal(reason),
        }
    }
}
