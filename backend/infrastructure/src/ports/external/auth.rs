use std::sync::Arc;

use async_trait::async_trait;
use domain::model::user::id::UserId;
use domain::ports::external::auth::{
    AuthError, AuthenticatorPort, CreatedIdentity, IdentityProviderPort, Principal,
};
use rs_firebase_admin_sdk::{
    App,
    auth::{FirebaseAuth, FirebaseAuthService, NewUser},
    client::{ReqwestApiClient, error::ApiClientError},
    jwt::{LiveValidator, TokenValidator, TokenVerificationError},
};
use serde_json::Value;
use tracing::error;

pub struct FirebaseAuthenticator {
    auth: FirebaseAuth<ReqwestApiClient>,
    token_validator: Arc<LiveValidator>,
}

impl FirebaseAuthenticator {
    pub fn new(project_id: &str) -> Self {
        let app = App::live_with_project_id(project_id.to_string())
            .expect("failed to initialize Firebase Admin SDK");
        let token_validator = Arc::new(
            LiveValidator::new_jwt_validator(project_id.to_string())
                .expect("failed to initialize Firebase token validator"),
        );
        Self {
            auth: app.auth(),
            token_validator,
        }
    }
}

#[async_trait]
impl AuthenticatorPort for FirebaseAuthenticator {
    async fn verify_id_token(&self, token: &str) -> Result<Principal, AuthError> {
        let claims = self
            .token_validator
            .clone()
            .validate(token.to_string())
            .await
            .map_err(map_token_error)?;

        Ok(Principal {
            uid: UserId::try_new(get_string_claim(&claims, "sub")?)
                .map_err(|_| AuthError::Unauthorized)?,
            email: get_string_claim(&claims, "email")?,
            issued_at: get_i64_claim(&claims, "iat")?,
            auth_time: get_i64_claim(&claims, "auth_time")?,
        })
    }
}

#[async_trait]
impl IdentityProviderPort for FirebaseAuthenticator {
    async fn create_user(&self, email: &str, password: &str) -> Result<CreatedIdentity, AuthError> {
        let user = self
            .auth
            .create_user(NewUser::email_and_password(
                email.to_string(),
                password.to_string(),
            ))
            .await
            .map_err(map_api_error)?;

        Ok(CreatedIdentity {
            uid: UserId::try_new(user.uid)
                .map_err(|error| AuthError::Unexpected(error.to_string()))?,
        })
    }

    async fn delete_user(&self, uid: &UserId) -> Result<(), AuthError> {
        match self.auth.delete_user(uid.as_str().to_string()).await {
            Ok(()) => Ok(()),
            Err(error) => {
                if is_user_not_found_error(&error) {
                    return Ok(());
                }
                Err(map_api_error(error))
            }
        }
    }
}

fn is_user_not_found_error(error: &error_stack::Report<ApiClientError>) -> bool {
    matches!(
        error.current_context(),
        ApiClientError::ServerError(api_error) if api_error.message == "USER_NOT_FOUND"
    )
}

fn get_string_claim(
    claims: &std::collections::HashMap<String, Value>,
    key: &str,
) -> Result<String, AuthError> {
    claims
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or(AuthError::Unauthorized)
}

fn get_i64_claim(
    claims: &std::collections::HashMap<String, Value>,
    key: &str,
) -> Result<i64, AuthError> {
    claims
        .get(key)
        .and_then(|value| {
            value
                .as_i64()
                .or_else(|| value.as_u64().and_then(|v| i64::try_from(v).ok()))
        })
        .ok_or(AuthError::Unauthorized)
}

fn map_token_error(error: error_stack::Report<TokenVerificationError>) -> AuthError {
    error!(error = ?error, "failed to verify Firebase ID token");
    match error.current_context() {
        TokenVerificationError::Invalid | TokenVerificationError::MissingKey => {
            AuthError::Unauthorized
        }
        TokenVerificationError::Internal => AuthError::TemporarilyUnavailable,
    }
}

fn map_api_error(error: error_stack::Report<ApiClientError>) -> AuthError {
    error!(error = ?error, "Firebase Auth API request failed");
    match error.current_context() {
        ApiClientError::ServerError(api_error) => match api_error.message.as_str() {
            "EMAIL_EXISTS" => AuthError::Conflict("email already exists".to_string()),
            "INVALID_EMAIL" => AuthError::BadRequest("invalid email".to_string()),
            "WEAK_PASSWORD" => AuthError::BadRequest("weak password".to_string()),
            _ if api_error.code >= 500 => AuthError::TemporarilyUnavailable,
            message => AuthError::Unexpected(format!("Firebase Auth error: {message}")),
        },
        ApiClientError::FailedToSendRequest | ApiClientError::FailedToReceiveResponse => {
            AuthError::TemporarilyUnavailable
        }
        ApiClientError::FailedToSerializeRequest | ApiClientError::FailedToDeserializeResponse => {
            AuthError::Unexpected("failed to handle Firebase Auth response".to_string())
        }
    }
}
