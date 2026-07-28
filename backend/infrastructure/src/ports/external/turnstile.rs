use std::time::Duration;

use async_trait::async_trait;
use domain::ports::external::turnstile::{TurnstileError, TurnstileVerifierPort};
use reqwest::Client;
use serde::Deserialize;
use tracing::{error, warn};

#[derive(Deserialize)]
struct SiteverifyResponse {
    success: bool,
    #[serde(default, rename = "error-codes")]
    error_codes: Vec<String>,
}

pub struct TurnstileVerifier {
    enabled: bool,
    secret_key: Option<String>,
    siteverify_endpoint: String,
    client: Client,
}

impl TurnstileVerifier {
    pub fn new(enabled: bool, secret_key: Option<String>, siteverify_endpoint: String) -> Self {
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(3))
            .timeout(Duration::from_secs(5))
            .build()
            .expect("reqwest client build failed");

        Self {
            enabled,
            secret_key,
            siteverify_endpoint,
            client,
        }
    }
}

#[async_trait]
impl TurnstileVerifierPort for TurnstileVerifier {
    async fn verify(&self, token: Option<&str>) -> Result<(), TurnstileError> {
        if !self.enabled {
            return Ok(());
        }

        let token = token
            .map(str::trim)
            .filter(|token| !token.is_empty())
            .ok_or(TurnstileError::TokenRequired)?;
        let secret_key = self
            .secret_key
            .as_deref()
            .ok_or_else(|| TurnstileError::Unexpected("turnstile secret key is missing".into()))?;

        let response = self
            .client
            .post(&self.siteverify_endpoint)
            .form(&[("secret", secret_key), ("response", token)])
            .send()
            .await
            .map_err(|error| {
                warn!(error = ?error, "failed to send Turnstile siteverify request");
                TurnstileError::TemporarilyUnavailable
            })?
            .error_for_status()
            .map_err(|error| {
                error!(error = ?error, "Turnstile siteverify returned error status");
                TurnstileError::TemporarilyUnavailable
            })?;

        let response = response
            .json::<SiteverifyResponse>()
            .await
            .map_err(|error| {
                error!(error = ?error, "failed to decode Turnstile siteverify response");
                TurnstileError::TemporarilyUnavailable
            })?;

        if response.success {
            return Ok(());
        }

        warn!(
            error_codes = ?response.error_codes,
            "Turnstile verification failed"
        );
        Err(TurnstileError::InvalidToken)
    }
}

#[cfg(test)]
mod tests {
    use domain::ports::external::turnstile::{TurnstileError, TurnstileVerifierPort};
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{body_string_contains, method, path},
    };

    use super::TurnstileVerifier;

    #[tokio::test]
    async fn verify_skips_request_when_disabled() {
        let verifier = TurnstileVerifier::new(false, None, "http://127.0.0.1:1".to_string());

        assert!(verifier.verify(None).await.is_ok());
    }

    #[tokio::test]
    async fn verify_accepts_successful_siteverify_response() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/"))
            .and(body_string_contains("secret=secret"))
            .and(body_string_contains("response=token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "success": true
            })))
            .mount(&server)
            .await;
        let verifier = TurnstileVerifier::new(true, Some("secret".into()), server.uri());

        assert!(verifier.verify(Some("token")).await.is_ok());
    }

    #[tokio::test]
    async fn verify_rejects_failed_siteverify_response() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "success": false,
                "error-codes": ["invalid-input-response"]
            })))
            .mount(&server)
            .await;
        let verifier = TurnstileVerifier::new(true, Some("secret".into()), server.uri());

        let error = verifier
            .verify(Some("invalid-token"))
            .await
            .expect_err("invalid");
        assert!(matches!(error, TurnstileError::InvalidToken));
    }
}
