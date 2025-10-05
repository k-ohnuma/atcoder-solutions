use async_trait::async_trait;
use domain::{
    model::atcoder_problems::ApiProblem, ports::external::atcoder_problems::AtcoderProblemsPort,
};
use reqwest::Client;
use shared::error::ExternalError;

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
    async fn fetch_problems(&self) -> Result<Vec<ApiProblem>, ExternalError> {
        let json_endpoint = format!("{}/resources/problems.json", self.base_endpoint);
        let resp = self
            .client
            .get(json_endpoint)
            .send()
            .await
            .map_err(ExternalError::from)?
            .error_for_status()
            .map_err(ExternalError::from)?;

        let json: Vec<ApiProblem> = resp
            .json()
            .await
            .map_err(ExternalError::from)?;
        Ok(json)
    }
}
