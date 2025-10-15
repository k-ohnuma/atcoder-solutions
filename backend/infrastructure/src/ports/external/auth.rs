use std::{
    collections::HashSet,
    sync::Arc,
    time::{Duration, SystemTime},
};

use async_trait::async_trait;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use domain::ports::external::auth::{AuthError, AuthenticatorPort, Principal};
use jsonwebtoken::{
    Algorithm, DecodingKey, Validation, decode, decode_header,
    jwk::{AlgorithmParameters, JwkSet},
};
use tokio::sync::RwLock;
use tracing::error;

#[derive(Clone)]
pub struct FirebaseAuthConfig {
    pub project_id: String,
    pub jwks_url: String,
}

#[derive(Clone)]
pub struct JwksCache {
    url: Arc<String>,
    inner: Arc<RwLock<Option<(JwkSet, SystemTime, Duration)>>>,
}
impl JwksCache {
    pub fn new(url: &str) -> Self {
        Self {
            url: Arc::new(url.into()),
            inner: Arc::new(RwLock::new(None)),
        }
    }
    pub async fn get(&self) -> Result<JwkSet, AuthError> {
        if let Some((set, at, ttl)) = self.inner.read().await.clone() {
            if at.elapsed().unwrap_or_default() < ttl {
                return Ok(set);
            }
        }
        let resp = reqwest::get(self.url.as_str())
            .await
            .map_err(|_| AuthError::TemporarilyUnavailable)?;
        let ttl = resp
            .headers()
            .get(reqwest::header::CACHE_CONTROL)
            .and_then(|v| v.to_str().ok())
            .and_then(|cc| {
                cc.split(',').find_map(|p| {
                    p.trim()
                        .strip_prefix("max-age=")
                        .and_then(|s| s.parse::<u64>().ok())
                })
            })
            .map(Duration::from_secs)
            .unwrap_or(Duration::from_secs(300));
        let set: JwkSet = resp
            .json()
            .await
            .map_err(|_| AuthError::TemporarilyUnavailable)?;
        *self.inner.write().await = Some((set.clone(), SystemTime::now(), ttl));
        Ok(set)
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
struct FirebaseClaims {
    sub: String,
    email: Option<String>,
}

#[derive(Clone)]
pub struct FirebaseAuthenticator {
    config: FirebaseAuthConfig,
    jwks: JwksCache,
}

impl FirebaseAuthenticator {
    pub fn new(project_id: &str) -> Self {
        let jwks_url = "https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com";
        Self {
            config: FirebaseAuthConfig {
                project_id: project_id.into(),
                jwks_url: jwks_url.into(),
            },
            jwks: JwksCache::new(jwks_url.into()),
        }
    }
}

#[async_trait]
impl AuthenticatorPort for FirebaseAuthenticator {
    async fn verify_id_token(&self, token: &str) -> Result<Principal, AuthError> {
        let header = decode_header(token).map_err(|_| AuthError::Unauthorized)?;
        let kid = header.kid.ok_or(AuthError::Unauthorized)?;

        let jwks = self.jwks.get().await?;
        let jwk = jwks.find(&kid).ok_or(AuthError::Unauthorized)?.to_owned();

        let (n, e) = match &jwk.algorithm {
            AlgorithmParameters::RSA(rsa) => (rsa.n.to_owned(), rsa.e.to_owned()),
            _ => return Err(AuthError::Unauthorized),
        };

        let n_bytes = URL_SAFE_NO_PAD
            .decode(n)
            .map_err(|_| AuthError::Unauthorized)?;
        let e_bytes = URL_SAFE_NO_PAD
            .decode(e)
            .map_err(|_| AuthError::Unauthorized)?;

        let key = DecodingKey::from_rsa_raw_components(&n_bytes, &e_bytes);

        let mut val = Validation::new(Algorithm::RS256);
        val.set_audience(&[self.config.project_id.as_str()]);
        val.iss = {
            let mut s = HashSet::new();
            s.insert(format!(
                "https://securetoken.google.com/{}",
                self.config.project_id
            ));
            Some(s)
        };
        let data = decode::<FirebaseClaims>(token, &key, &val).map_err(|e| {
            error!(error = ?e);
            AuthError::Unauthorized
        })?;

        Ok(Principal {
            uid: data.claims.sub,
            email: data.claims.email.ok_or(AuthError::Unauthorized)?,
        })
    }
}
