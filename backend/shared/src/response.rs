use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub ok: bool,
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "errorCode")]
    pub error_code: Option<String>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn ok(data: T) -> Self {
        Self {
            ok: true,
            status: StatusCode::OK.as_u16(),
            data: Some(data),
            error: None,
            error_code: None,
        }
    }

    pub fn err(status: StatusCode, msg: impl Into<String>) -> Self {
        Self::err_with_code(status, msg, None::<String>)
    }

    pub fn err_with_code(
        status: StatusCode,
        msg: impl Into<String>,
        error_code: impl Into<Option<String>>,
    ) -> Self {
        Self {
            ok: false,
            status: status.as_u16(),
            data: None,
            error: Some(msg.into()),
            error_code: error_code.into(),
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}
