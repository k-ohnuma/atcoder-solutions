use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;
use thiserror::Error;

use crate::response::ApiResponse;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal Server Error: {0}")]
    Internal(String),
}

impl HttpError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            HttpError::BadRequest(_) => StatusCode::BAD_REQUEST,
            HttpError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            HttpError::Forbidden(_) => StatusCode::FORBIDDEN,
            HttpError::NotFound(_) => StatusCode::NOT_FOUND,
            HttpError::Conflict(_) => StatusCode::CONFLICT,
            HttpError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            HttpError::BadRequest(_) => "BAD_REQUEST",
            HttpError::Unauthorized(_) => "UNAUTHORIZED",
            HttpError::Forbidden(_) => "FORBIDDEN",
            HttpError::NotFound(_) => "NOT_FOUND",
            HttpError::Conflict(_) => "CONFLICT",
            HttpError::Internal(_) => "INTERNAL",
        }
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code();
        let error_code = self.error_code();
        match status {
            s if s.is_server_error() => {
                tracing::error!(http.status = s.as_u16(), error.code = error_code, error.message = %self, "http error response");
            }
            s if s.is_client_error() => {
                tracing::warn!(http.status = s.as_u16(), error.code = error_code, error.message = %self, "http error response");
            }
            s => {
                tracing::info!(http.status = s.as_u16(), error.code = error_code, error.message = %self, "http error response");
            }
        }
        let public_message = if status.is_server_error() {
            "Internal Server Error".to_string()
        } else {
            self.to_string()
        };
        let body: ApiResponse<()> =
            ApiResponse::err_with_code(status, public_message, Some(error_code.to_string()));
        body.into_response()
    }
}
