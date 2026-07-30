use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts},
    response::IntoResponse,
};
use domain::{
    model::user::Role,
    ports::external::auth::{AuthError, Principal},
};
use registry::Registry;

use crate::error::HttpError;

pub struct AuthUser(pub Principal);
pub struct VerifiedUser(pub Principal);
pub struct AdminUser(pub Principal);

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
        let token = bearer_token(parts)?;

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
        let token = bearer_token(parts)?;

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

        match user.role {
            Role::Admin => Ok(AdminUser(principal)),
            _ => Err(AuthRejection::Forbidden),
        }
    }
}

fn bearer_token(parts: &Parts) -> Result<&str, AuthRejection> {
    let authz = parts
        .headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or(AuthRejection::Unauthorized)?;
    authz
        .strip_prefix("Bearer ")
        .ok_or(AuthRejection::Unauthorized)
}
