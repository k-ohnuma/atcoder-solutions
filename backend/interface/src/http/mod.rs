use axum::{
    extract::{FromRequest, FromRequestParts, Json, Query, Request},
    http::{StatusCode, header, request::Parts},
    response::IntoResponse,
};
use domain::{
    model::user::Role,
    ports::external::auth::{AuthError, Principal},
};
use serde::de::DeserializeOwned;
use shared::{error::http::HttpError, response::ApiResponse};

use registry::Registry;

pub struct AuthUser(pub Principal);
pub struct AdminUser(pub Principal);
pub struct ApiJson<T>(pub T);
pub struct ApiQuery<T>(pub T);

#[derive(Debug)]
pub enum AuthRejection {
    Unauthorized,
    Unavailable,
    Forbidden,
}
impl IntoResponse for AuthRejection {
    fn into_response(self) -> axum::response::Response {
        let (status, message, error_code) = match self {
            AuthRejection::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "Unauthorized",
                "UNAUTHORIZED",
            ),
            AuthRejection::Unavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Service Unavailable",
                "SERVICE_UNAVAILABLE",
            ),
            AuthRejection::Forbidden => (StatusCode::FORBIDDEN, "Forbidden", "FORBIDDEN"),
        };
        ApiResponse::<()>::err_with_code(status, message, Some(error_code.to_string()))
            .into_response()
    }
}

impl FromRequestParts<Registry> for AuthUser {
    type Rejection = AuthRejection;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &Registry,
    ) -> Result<Self, Self::Rejection> {
        let authz = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(AuthRejection::Unauthorized)?;
        let token = authz
            .strip_prefix("Bearer ")
            .ok_or(AuthRejection::Unauthorized)?;

        match state.auth_port().verify_id_token(token).await {
            Ok(p) => {
                let revoked = state
                    .user_repository()
                    .is_token_revoked(&p.uid, p.issued_at)
                    .await
                    .map_err(|_| AuthRejection::Unavailable)?;
                if revoked {
                    return Err(AuthRejection::Unauthorized);
                }
                Ok(AuthUser(p))
            }
            Err(AuthError::Unauthorized) => Err(AuthRejection::Unauthorized),
            Err(AuthError::TemporarilyUnavailable) => Err(AuthRejection::Unavailable),
        }
    }
}

impl FromRequestParts<Registry> for AdminUser {
    type Rejection = AuthRejection;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &Registry,
    ) -> Result<Self, Self::Rejection> {
        let AuthUser(principal) = AuthUser::from_request_parts(parts, state).await?;
        let user = state
            .user_repository()
            .find_by_uid(principal.uid.as_str())
            .await
            .map_err(|_c| AuthRejection::Unavailable)?;

        let role = user.role;

        match role {
            Role::Admin => Ok(AdminUser(principal)),
            _ => Err(AuthRejection::Forbidden),
        }
    }
}

impl<S, T> FromRequest<S> for ApiJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Send,
{
    type Rejection = HttpError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        Json::<T>::from_request(req, state)
            .await
            .map(|Json(value)| ApiJson(value))
            .map_err(|e| HttpError::BadRequest(e.body_text()))
    }
}

impl<S, T> FromRequestParts<S> for ApiQuery<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Send,
{
    type Rejection = HttpError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        Query::<T>::from_request_parts(parts, state)
            .await
            .map(|Query(value)| ApiQuery(value))
            .map_err(|e| HttpError::BadRequest(e.to_string()))
    }
}
