use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts},
    response::IntoResponse,
};
use domain::{
    model::user::Role,
    ports::external::auth::{AuthProviderError, Principal, VerifyTokenError},
};
use registry::Registry;

use crate::error::HttpError;

pub struct AuthUser(pub Principal);
pub struct VerifiedUser(pub Principal);
pub struct AdminUser(pub Principal);

#[derive(Debug)]
pub enum AuthRejection {
    Unauthorized,
    Unavailable(String),
    Internal(String),
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
            AuthRejection::Unavailable(reason) => HttpError::ServiceUnavailable(reason),
            AuthRejection::Internal(reason) => HttpError::Internal(reason),
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

        let principal = verify_principal(state, token).await?;
        let revoked = state
            .user_repository()
            .is_token_revoked(&principal.uid, principal.issued_at)
            .await
            .map_err(|error| {
                AuthRejection::Unavailable(format!("failed to check token revocation: {error}"))
            })?;
        if revoked {
            return Err(AuthRejection::Unauthorized);
        }

        Ok(AuthUser(principal))
    }
}

impl FromRequestParts<Registry> for VerifiedUser {
    type Rejection = AuthRejection;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &Registry,
    ) -> Result<Self, Self::Rejection> {
        let token = bearer_token(parts)?;

        verify_principal(state, token).await.map(VerifiedUser)
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
            .find_by_uid(&principal.uid)
            .await
            .map_err(|error| {
                AuthRejection::Unavailable(format!("failed to load authenticated user: {error}"))
            })?
            .ok_or(AuthRejection::Forbidden)?;

        match user.role {
            Role::Admin => Ok(AdminUser(principal)),
            _ => Err(AuthRejection::Forbidden),
        }
    }
}

async fn verify_principal(state: &Registry, token: &str) -> Result<Principal, AuthRejection> {
    state
        .auth_port()
        .verify_id_token(token)
        .await
        .map_err(map_token_verification_error)
}

fn map_token_verification_error(error: VerifyTokenError) -> AuthRejection {
    match error {
        VerifyTokenError::InvalidToken => AuthRejection::Unauthorized,
        VerifyTokenError::Provider(AuthProviderError::TemporarilyUnavailable) => {
            AuthRejection::Unavailable(
                "failed to verify ID token: temporarily unavailable".to_string(),
            )
        }
        VerifyTokenError::Provider(AuthProviderError::Unexpected(reason)) => {
            AuthRejection::Internal(format!("failed to verify ID token: {reason}"))
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

#[cfg(test)]
mod tests {
    use domain::ports::external::auth::{AuthProviderError, VerifyTokenError};

    use super::{AuthRejection, map_token_verification_error};

    #[test]
    fn unexpected_auth_error_maps_to_internal_and_keeps_diagnostic_context() {
        let rejection = map_token_verification_error(VerifyTokenError::Provider(
            AuthProviderError::Unexpected("key service failed".to_string()),
        ));

        assert!(matches!(
            rejection,
            AuthRejection::Internal(reason)
                if reason == "failed to verify ID token: key service failed"
        ));
    }

    #[test]
    fn invalid_token_details_are_not_exposed() {
        let rejection = map_token_verification_error(VerifyTokenError::InvalidToken);

        assert!(matches!(rejection, AuthRejection::Unauthorized));
    }

    #[test]
    fn temporarily_unavailable_auth_provider_maps_to_unavailable() {
        let rejection = map_token_verification_error(VerifyTokenError::Provider(
            AuthProviderError::TemporarilyUnavailable,
        ));

        assert!(matches!(rejection, AuthRejection::Unavailable(_)));
    }
}
