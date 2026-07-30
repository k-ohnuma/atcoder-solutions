use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

use crate::response::ApiResponse;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Unauthorized: {message}")]
    UnauthorizedWithCode {
        message: String,
        error_code: &'static str,
    },

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Service Unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Conflict: {message}")]
    ConflictWithCode {
        message: String,
        error_code: &'static str,
    },

    #[error("Internal Server Error: {0}")]
    Internal(String),
}

impl HttpError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            HttpError::BadRequest(_) => StatusCode::BAD_REQUEST,
            HttpError::Unauthorized(_) | HttpError::UnauthorizedWithCode { .. } => {
                StatusCode::UNAUTHORIZED
            }
            HttpError::Forbidden(_) => StatusCode::FORBIDDEN,
            HttpError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            HttpError::NotFound(_) => StatusCode::NOT_FOUND,
            HttpError::Conflict(_) | HttpError::ConflictWithCode { .. } => StatusCode::CONFLICT,
            HttpError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            HttpError::BadRequest(_) => "BAD_REQUEST",
            HttpError::Unauthorized(_) => "UNAUTHORIZED",
            HttpError::UnauthorizedWithCode { error_code, .. } => error_code,
            HttpError::Forbidden(_) => "FORBIDDEN",
            HttpError::ServiceUnavailable(_) => "SERVICE_UNAVAILABLE",
            HttpError::NotFound(_) => "NOT_FOUND",
            HttpError::Conflict(_) => "CONFLICT",
            HttpError::ConflictWithCode { error_code, .. } => error_code,
            HttpError::Internal(_) => "INTERNAL",
        }
    }
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

#[cfg(test)]
mod tests {
    use axum::{body::to_bytes, http::StatusCode, response::IntoResponse};
    use serde_json::Value;

    use super::HttpError;

    async fn response_json(error: HttpError) -> (StatusCode, Value) {
        let response = error.into_response();
        let status = response.status();
        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("response body");
        let json = serde_json::from_slice(&body).expect("json body");
        (status, json)
    }

    #[tokio::test]
    async fn client_error_response_has_common_shape() {
        let (status, json) =
            response_json(HttpError::BadRequest("invalid input".to_string())).await;

        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(json["ok"], false);
        assert_eq!(json["status"], 400);
        assert_eq!(json["error"], "Bad Request: invalid input");
        assert_eq!(json["errorCode"], "BAD_REQUEST");
    }

    #[tokio::test]
    async fn coded_error_response_has_common_shape() {
        let (status, json) = response_json(HttpError::ConflictWithCode {
            message: "email already exists".to_string(),
            error_code: "EMAIL_ALREADY_EXISTS",
        })
        .await;

        assert_eq!(status, StatusCode::CONFLICT);
        assert_eq!(json["ok"], false);
        assert_eq!(json["status"], 409);
        assert_eq!(json["error"], "Conflict: email already exists");
        assert_eq!(json["errorCode"], "EMAIL_ALREADY_EXISTS");
    }

    #[tokio::test]
    async fn server_error_response_hides_internal_message() {
        let (status, json) =
            response_json(HttpError::Internal("database password leaked".to_string())).await;

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(json["ok"], false);
        assert_eq!(json["status"], 500);
        assert_eq!(json["error"], "Internal Server Error");
        assert_eq!(json["errorCode"], "INTERNAL");
    }
}
