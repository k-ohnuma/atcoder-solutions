use std::time::Duration;

use async_trait::async_trait;
use domain::{
    error::external::ExternalError, model::problem::Problem,
    ports::external::atcoder_problems::AtcoderProblemsPort,
};
use reqwest::Client;
use serde::Deserialize;
use tracing::warn;

use crate::error::map_reqwest_error;

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
struct ApiProblem {
    id: String,
    contest_id: String,
    problem_index: String,
    name: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ApiProblemDifficulty {
    difficulty: Option<i32>,
}

impl From<ApiProblem> for Problem {
    fn from(value: ApiProblem) -> Self {
        Self {
            id: value.id,
            contest_code: value.contest_id,
            problem_index: value.problem_index,
            title: value.name,
            difficulty: None,
        }
    }
}

pub struct AtcoderProblemsClient {
    client: Client,
    base_endpoint: String,
    difficulty_endpoint: String,
}

impl AtcoderProblemsClient {
    pub fn new(base: &str, difficulty_endpoint: &str) -> Self {
        let client = Client::builder()
            .gzip(true)
            .brotli(true)
            .deflate(true)
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(30))
            .build()
            .expect("reqwest client build failed");

        Self {
            client,
            base_endpoint: base.into(),
            difficulty_endpoint: difficulty_endpoint.into(),
        }
    }

    async fn fetch_difficulty(
        &self,
        problem_id: &str,
    ) -> Result<Option<i32>, ExternalError> {
        let resp = self
            .client
            .get(self.difficulty_endpoint.as_str())
            .query(&[("problemId", problem_id)])
            .send()
            .await
            .map_err(map_reqwest_error)?;

        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }
        let resp = resp.error_for_status().map_err(map_reqwest_error)?;

        let payload = resp.json::<ApiProblemDifficulty>().await.map_err(|e| {
            if e.is_decode() {
                ExternalError::InvalidJson(e.to_string())
            } else {
                map_reqwest_error(e)
            }
        })?;
        Ok(payload.difficulty)
    }
}

#[async_trait]
impl AtcoderProblemsPort for AtcoderProblemsClient {
    async fn fetch_problems(&self) -> Result<Vec<Problem>, ExternalError> {
        let json_endpoint = format!("{}/resources/problems.json", self.base_endpoint);
        let resp = self
            .client
            .get(json_endpoint)
            .send()
            .await
            .map_err(map_reqwest_error)?
            .error_for_status()
            .map_err(map_reqwest_error)?;

        let json: Vec<ApiProblem> = resp.json().await.map_err(|e| {
            if e.is_decode() {
                ExternalError::InvalidJson(e.to_string())
            } else {
                map_reqwest_error(e)
            }
        })?;
        Ok(json.into_iter().map(Problem::from).collect::<Vec<Problem>>())
    }

    async fn fetch_difficulty(&self, problem_id: &str) -> Result<Option<i32>, ExternalError> {
        self.fetch_difficulty(problem_id).await.inspect_err(|error| {
            warn!(problem_id, error = ?error, "difficulty fetch failed");
        })
    }
}

#[cfg(test)]
mod tests {

    use domain::{
        error::external::ExternalError, ports::external::atcoder_problems::AtcoderProblemsPort,
    };
    use rstest::{fixture, rstest};
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path},
    };

    use super::AtcoderProblemsClient;

    #[fixture]
    async fn server_and_client() -> (MockServer, AtcoderProblemsClient) {
        let server = MockServer::start().await;
        let client = AtcoderProblemsClient::new(server.uri().as_str(), server.uri().as_str());
        (server, client)
    }

    #[rstest]
    #[tokio::test]
    async fn fetch_problems_ok(
        #[future(awt)] server_and_client: (MockServer, AtcoderProblemsClient),
    ) {
        let mock_server = server_and_client.0;
        let client = server_and_client.1;
        let body_str = include_str!("../../../../tests/fixtures/problems.json");
        let body_val: serde_json::Value = serde_json::from_str(body_str).unwrap();

        Mock::given(method("GET"))
            .and(path("/resources/problems.json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body_val))
            .mount(&mock_server)
            .await;
        let got = client.fetch_problems().await.expect("should be success");
        assert_eq!(got.len(), 3);

        let p1 = &got[0];
        assert_eq!(p1.id, "abc234_a");
        assert_eq!(p1.contest_code, "abc234");
        assert_eq!(p1.problem_index, "A");
        assert_eq!(p1.title, "Weird Function");
        assert_eq!(p1.difficulty, None);
    }

    #[rstest]
    #[case(404, ExternalError::NotFound)]
    #[case(500, ExternalError::ExternalUnavailable)]
    #[tokio::test]
    async fn fetch_problems_http_errors(
        #[future(awt)] server_and_client: (MockServer, AtcoderProblemsClient),
        #[case] status: u16,
        #[case] _expected_variant: ExternalError,
    ) {
        let server = server_and_client.0;
        let client = server_and_client.1;
        Mock::given(method("GET"))
            .and(path("/resources/problems.json"))
            .respond_with(ResponseTemplate::new(status))
            .mount(&server)
            .await;

        let err = client.fetch_problems().await.expect_err("should fail");

        match (status, err) {
            (404, ExternalError::NotFound) => {}
            (500..=599, ExternalError::ExternalUnavailable) => {}
            (s, e) => panic!("unexpected mapping: status={s}, err={e:?}"),
        }
    }

    #[rstest]
    #[tokio::test]
    async fn fetch_problems_invalid_json(
        #[future(awt)] server_and_client: (MockServer, AtcoderProblemsClient),
    ) {
        let server = server_and_client.0;
        let client = server_and_client.1;
        Mock::given(method("GET"))
            .and(path("/resources/problems.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string("{ not json ]"))
            .mount(&server)
            .await;

        let err = client.fetch_problems().await.expect_err("should fail");
        println!("{:?}", err);
        assert!(matches!(err, ExternalError::InvalidJson(_)));
    }

    #[rstest]
    #[tokio::test]
    async fn fetch_difficulty_ok(
        #[future(awt)] server_and_client: (MockServer, AtcoderProblemsClient),
    ) {
        let server = server_and_client.0;
        let client = server_and_client.1;
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({ "difficulty": 1279 })))
            .mount(&server)
            .await;

        let got = client.fetch_difficulty("abc323_e").await.expect("should be success");
        assert_eq!(got, Some(1279));
    }

    #[rstest]
    #[tokio::test]
    async fn fetch_difficulty_not_found_returns_none(
        #[future(awt)] server_and_client: (MockServer, AtcoderProblemsClient),
    ) {
        let server = server_and_client.0;
        let client = server_and_client.1;
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let got = client.fetch_difficulty("no_such_problem").await.expect("should be success");
        assert_eq!(got, None);
    }
}
