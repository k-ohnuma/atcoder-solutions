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

fn contest_code_from_problem_id(problem_id: &str, fallback_contest_id: &str) -> String {
    problem_id
        .rsplit_once('_')
        .map(|(contest_code, _)| contest_code.to_string())
        .unwrap_or_else(|| fallback_contest_id.to_string())
}

fn problem_index_from_problem_id(problem_id: &str, fallback_problem_index: &str) -> String {
    problem_id
        .rsplit_once('_')
        .map(|(_, problem_index)| problem_index.to_ascii_uppercase())
        .unwrap_or_else(|| fallback_problem_index.to_string())
}

impl From<ApiProblem> for Problem {
    fn from(value: ApiProblem) -> Self {
        let contest_code = contest_code_from_problem_id(&value.id, &value.contest_id);
        let problem_index = problem_index_from_problem_id(&value.id, &value.problem_index);
        Self {
            id: value.id,
            contest_code,
            problem_index,
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

    async fn fetch_difficulty(&self, problem_id: &str) -> Result<Option<i32>, ExternalError> {
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
        Ok(json
            .into_iter()
            .map(Problem::from)
            .collect::<Vec<Problem>>())
    }

    async fn fetch_difficulty(&self, problem_id: &str) -> Result<Option<i32>, ExternalError> {
        self.fetch_difficulty(problem_id)
            .await
            .inspect_err(|error| {
                warn!(problem_id, error = ?error, "difficulty fetch failed");
            })
    }
}

#[cfg(test)]
mod tests {
    use domain::{
        error::external::ExternalError, model::problem::Problem,
        ports::external::atcoder_problems::AtcoderProblemsPort,
    };
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path},
    };

    use super::{
        ApiProblem, AtcoderProblemsClient, contest_code_from_problem_id,
        problem_index_from_problem_id,
    };

    async fn server_and_client() -> (MockServer, AtcoderProblemsClient) {
        let server = MockServer::start().await;
        let client = AtcoderProblemsClient::new(server.uri().as_str(), server.uri().as_str());
        (server, client)
    }

    #[tokio::test]
    async fn fetch_problems_ok() {
        let server_and_client = server_and_client().await;
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

    #[test]
    fn contest_code_is_derived_from_problem_id() {
        for (problem_id, contest_id, expected) in [
            ("abc395_a", "adt_easy_20250430_3", "abc395"),
            ("abc001_1", "abc001", "abc001"),
            ("tessoku_book_fj", "tessoku-book", "tessoku_book"),
            ("APG4bPython_ak", "APG4bPython", "APG4bPython"),
            (
                "problem-without-separator",
                "fallback_contest",
                "fallback_contest",
            ),
        ] {
            assert_eq!(
                contest_code_from_problem_id(problem_id, contest_id),
                expected
            );
        }
    }

    #[test]
    fn problem_index_is_derived_from_problem_id() {
        for (problem_id, problem_index, expected) in [
            ("abc395_a", "B", "A"),
            ("abc007_1", "A", "1"),
            ("abc007_3", "A", "3"),
            ("tessoku_book_fj", "a1", "FJ"),
            ("problem-without-separator", "A", "A"),
        ] {
            assert_eq!(
                problem_index_from_problem_id(problem_id, problem_index),
                expected
            );
        }
    }

    #[test]
    fn api_problem_conversion_uses_original_contest_code_and_index_from_problem_id() {
        let problem = Problem::from(ApiProblem {
            id: "abc395_a".into(),
            contest_id: "adt_easy_20250430_3".into(),
            problem_index: "B".into(),
            name: "Strictly Increasing?".into(),
        });

        assert_eq!(problem.id, "abc395_a");
        assert_eq!(problem.contest_code, "abc395");
        assert_eq!(problem.problem_index, "A");
        assert_eq!(problem.title, "Strictly Increasing?");
        assert_eq!(problem.difficulty, None);
    }

    #[tokio::test]
    async fn fetch_problems_http_errors() {
        for status in [404, 500] {
            let (server, client) = server_and_client().await;
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
    }

    #[tokio::test]
    async fn fetch_problems_invalid_json() {
        let server_and_client = server_and_client().await;
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

    #[tokio::test]
    async fn fetch_difficulty_ok() {
        let server_and_client = server_and_client().await;
        let server = server_and_client.0;
        let client = server_and_client.1;
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({ "difficulty": 1279 })),
            )
            .mount(&server)
            .await;

        let got = client
            .fetch_difficulty("abc323_e")
            .await
            .expect("should be success");
        assert_eq!(got, Some(1279));
    }

    #[tokio::test]
    async fn fetch_difficulty_not_found_returns_none() {
        let server_and_client = server_and_client().await;
        let server = server_and_client.0;
        let client = server_and_client.1;
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let got = client
            .fetch_difficulty("no_such_problem")
            .await
            .expect("should be success");
        assert_eq!(got, None);
    }
}
