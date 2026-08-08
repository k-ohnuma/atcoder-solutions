use std::sync::Arc;

use async_trait::async_trait;
use domain::model::user::id::UserId;
use domain::ports::external::auth::{
    AuthProviderError, AuthenticatorPort, CreateIdentityError, CreatedIdentity,
    IdentityProviderPort, Principal, VerifyTokenError,
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
    async fn verify_id_token(&self, token: &str) -> Result<Principal, VerifyTokenError> {
        let claims = self
            .token_validator
            .clone()
            .validate(token.to_string())
            .await
            .map_err(map_token_error)?;

        Ok(Principal {
            uid: UserId::try_new(get_string_claim(&claims, "sub")?)
                .map_err(|_| VerifyTokenError::InvalidToken)?,
            email: get_string_claim(&claims, "email")?,
            issued_at: get_i64_claim(&claims, "iat")?,
            auth_time: get_i64_claim(&claims, "auth_time")?,
        })
    }
}

#[async_trait]
impl IdentityProviderPort for FirebaseAuthenticator {
    async fn create_user(
        &self,
        email: &str,
        password: &str,
    ) -> Result<CreatedIdentity, CreateIdentityError> {
        let user = self
            .auth
            .create_user(NewUser::email_and_password(
                email.to_string(),
                password.to_string(),
            ))
            .await
            .map_err(map_create_identity_error)?;

        Ok(CreatedIdentity {
            uid: UserId::try_new(user.uid).map_err(|error| {
                CreateIdentityError::Provider(AuthProviderError::Unexpected(error.to_string()))
            })?,
        })
    }

    async fn delete_user(&self, uid: &UserId) -> Result<(), AuthProviderError> {
        match self.auth.delete_user(uid.as_str().to_string()).await {
            Ok(()) => Ok(()),
            Err(error) => {
                if is_user_not_found_error(&error) {
                    return Ok(());
                }
                Err(map_provider_error(error))
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
) -> Result<String, VerifyTokenError> {
    claims
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or(VerifyTokenError::InvalidToken)
}

fn get_i64_claim(
    claims: &std::collections::HashMap<String, Value>,
    key: &str,
) -> Result<i64, VerifyTokenError> {
    claims
        .get(key)
        .and_then(|value| {
            value
                .as_i64()
                .or_else(|| value.as_u64().and_then(|v| i64::try_from(v).ok()))
        })
        .ok_or(VerifyTokenError::InvalidToken)
}

fn map_token_error(error: error_stack::Report<TokenVerificationError>) -> VerifyTokenError {
    error!(error = ?error, "failed to verify Firebase ID token");
    match error.current_context() {
        TokenVerificationError::Invalid | TokenVerificationError::MissingKey => {
            VerifyTokenError::InvalidToken
        }
        TokenVerificationError::Internal => {
            VerifyTokenError::Provider(AuthProviderError::TemporarilyUnavailable)
        }
    }
}

fn map_create_identity_error(error: error_stack::Report<ApiClientError>) -> CreateIdentityError {
    error!(error = ?error, "Firebase Auth create user request failed");
    match error.current_context() {
        ApiClientError::ServerError(api_error) => match api_error.message.as_str() {
            "EMAIL_EXISTS" => CreateIdentityError::EmailAlreadyExists,
            "INVALID_EMAIL" => CreateIdentityError::InvalidEmail,
            "WEAK_PASSWORD" => CreateIdentityError::WeakPassword,
            _ if api_error.code >= 500 => {
                CreateIdentityError::Provider(AuthProviderError::TemporarilyUnavailable)
            }
            message => CreateIdentityError::Provider(AuthProviderError::Unexpected(format!(
                "Firebase Auth error: {message}"
            ))),
        },
        ApiClientError::FailedToSendRequest | ApiClientError::FailedToReceiveResponse => {
            CreateIdentityError::Provider(AuthProviderError::TemporarilyUnavailable)
        }
        ApiClientError::FailedToSerializeRequest | ApiClientError::FailedToDeserializeResponse => {
            CreateIdentityError::Provider(AuthProviderError::Unexpected(
                "failed to handle Firebase Auth response".to_string(),
            ))
        }
    }
}

fn map_provider_error(error: error_stack::Report<ApiClientError>) -> AuthProviderError {
    error!(error = ?error, "Firebase Auth API request failed");
    match error.current_context() {
        ApiClientError::ServerError(api_error) if api_error.code >= 500 => {
            AuthProviderError::TemporarilyUnavailable
        }
        ApiClientError::ServerError(api_error) => {
            AuthProviderError::Unexpected(format!("Firebase Auth error: {}", api_error.message))
        }
        ApiClientError::FailedToSendRequest | ApiClientError::FailedToReceiveResponse => {
            AuthProviderError::TemporarilyUnavailable
        }
        ApiClientError::FailedToSerializeRequest | ApiClientError::FailedToDeserializeResponse => {
            AuthProviderError::Unexpected("failed to handle Firebase Auth response".to_string())
        }
    }
}
