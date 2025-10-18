use axum::{
    extract::FromRequestParts,
    http::{StatusCode, header, request::Parts},
    response::IntoResponse,
};
use domain::{
    model::user::Role,
    ports::external::auth::{AuthError, Principal},
};

use registry::Registry;

pub struct AuthUser(pub Principal);
pub struct AdminUser(pub Principal);

#[derive(Debug)]
pub enum AuthRejection {
    Unauthorized,
    Unavailable,
    Forbidden,
}
impl IntoResponse for AuthRejection {
    fn into_response(self) -> axum::response::Response {
        let code = match self {
            AuthRejection::Unauthorized => StatusCode::UNAUTHORIZED,
            AuthRejection::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
            AuthRejection::Forbidden => StatusCode::FORBIDDEN,
        };
        (code, code.canonical_reason().unwrap_or("")).into_response()
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
            Ok(p) => Ok(AuthUser(p)),
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
