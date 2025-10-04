use async_trait::async_trait;
use domain::{
    model::atcoder_problems::ApiProblem,
    ports::external::atcoder_problems::{AtcoderProblemsError, AtcoderProblemsPort},
};
use reqwest::{Client, StatusCode};

fn map_requwest_err(err: reqwest::Error) -> AtcoderProblemsError {
    if err.is_connect() || err.is_timeout() {
        return AtcoderProblemsError::ExternalUnavailable;
    }

    if let Some(s) = err.status() {
        return match s {
            StatusCode::NOT_FOUND => AtcoderProblemsError::NotFound,
            _ => AtcoderProblemsError::ExternalUnavailable,
        };
    }

    AtcoderProblemsError::Other(err.into())
}

pub struct AtcoderProblemsClient {
    client: Client,
    base_endpoint: String,
}

impl AtcoderProblemsClient {
    pub fn new(base: &str) -> Self {
        Self {
            client: Client::new(),
            base_endpoint: base.into(),
        }
    }
}

#[async_trait]
impl AtcoderProblemsPort for AtcoderProblemsClient {
    async fn fetch_problems(&self) -> Result<Vec<ApiProblem>, AtcoderProblemsError> {
        let json_endpoint = format!("{}/resources/problems.json", self.base_endpoint);
        let resp = self
            .client
            .get(json_endpoint)
            .send()
            .await
            .map_err(|e| map_requwest_err(e))?
            .error_for_status()
            .map_err(|e| map_requwest_err(e))?;

        let json: Vec<ApiProblem> = resp
            .json()
            .await
            .map_err(|_e| AtcoderProblemsError::InvalidJson)?;
        Ok(json)
    }
}
