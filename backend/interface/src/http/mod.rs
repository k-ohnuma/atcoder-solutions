use axum::{
    extract::{FromRequest, FromRequestParts, Json, Query, Request},
    http::{header, request::Parts},
    response::IntoResponse,
};
use domain::{
    model::user::Role,
    ports::external::{
        auth::{AuthError, Principal},
        turnstile::TurnstileError,
    },
};
use serde::de::DeserializeOwned;

use registry::Registry;

use crate::error::HttpError;
use crate::response::ApiResponse;

pub struct AuthUser(pub Principal);
pub struct VerifiedUser(pub Principal);
pub struct AdminUser(pub Principal);
pub struct TurnstileGuard;
pub struct ApiJson<T>(pub T);
pub struct ApiQuery<T>(pub T);

pub type ApiResult<T> = Result<ApiResponse<T>, HttpError>;

#[derive(Debug)]
pub enum AuthRejection {
    Unauthorized,
    Unavailable,
    Forbidden,
}
impl IntoResponse for AuthRejection {
    fn into_response(self) -> axum::response::Response {
        HttpError::from(self).into_response()
    }
}

impl From<AuthRejection> for HttpError {
    fn from(value: AuthRejection) -> Self {
        match value {
            AuthRejection::Unauthorized => HttpError::Unauthorized("Unauthorized".to_string()),
            AuthRejection::Unavailable => {
                HttpError::ServiceUnavailable("Service Unavailable".to_string())
            }
            AuthRejection::Forbidden => HttpError::Forbidden("Forbidden".to_string()),
        }
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
            Err(AuthError::BadRequest(_)) | Err(AuthError::Conflict(_)) => {
                Err(AuthRejection::Unauthorized)
            }
            Err(AuthError::TemporarilyUnavailable) => Err(AuthRejection::Unavailable),
            Err(AuthError::Unexpected(_)) => Err(AuthRejection::Unavailable),
        }
    }
}

impl FromRequestParts<Registry> for VerifiedUser {
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
            Ok(p) => Ok(VerifiedUser(p)),
            Err(AuthError::Unauthorized) => Err(AuthRejection::Unauthorized),
            Err(AuthError::BadRequest(_)) | Err(AuthError::Conflict(_)) => {
                Err(AuthRejection::Unauthorized)
            }
            Err(AuthError::TemporarilyUnavailable) => Err(AuthRejection::Unavailable),
            Err(AuthError::Unexpected(_)) => Err(AuthRejection::Unavailable),
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

impl FromRequestParts<Registry> for TurnstileGuard {
    type Rejection = AuthRejection;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &Registry,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get("x-turnstile-token")
            .and_then(|v| v.to_str().ok());

        state
            .turnstile_verifier_port()
            .verify(token)
            .await
            .map_err(|error| match error {
                TurnstileError::TokenRequired | TurnstileError::InvalidToken => {
                    AuthRejection::Forbidden
                }
                TurnstileError::TemporarilyUnavailable | TurnstileError::Unexpected(_) => {
                    AuthRejection::Unavailable
                }
            })?;

        Ok(TurnstileGuard)
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

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Query::<T>::from_request_parts(parts, state)
            .await
            .map(|Query(value)| ApiQuery(value))
            .map_err(|e| HttpError::BadRequest(e.to_string()))
    }
}
