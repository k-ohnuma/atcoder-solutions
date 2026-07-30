use axum::{
    extract::FromRequestParts,
    http::{header::HeaderName, request::Parts},
};
use domain::ports::external::turnstile::TurnstileError;
use registry::Registry;

use super::auth::AuthRejection;

const TURNSTILE_TOKEN_HEADER: HeaderName = HeaderName::from_static("x-turnstile-token");

pub struct TurnstileGuard;

impl FromRequestParts<Registry> for TurnstileGuard {
    type Rejection = AuthRejection;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &Registry,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(TURNSTILE_TOKEN_HEADER)
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
