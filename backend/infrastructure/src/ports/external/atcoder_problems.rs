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

        let json: Vec<ApiProblem> = resp.json().await.map_err(|e| {
            if e.is_decode() {
                ExternalError::InvalidJson(e.to_string())
            } else {
                ExternalError::from(e)
            }
        })?;
        Ok(json)
    }
}

#[cfg(test)]
mod tests {

    use domain::{
        model::atcoder_problems::ApiProblem, ports::external::atcoder_problems::AtcoderProblemsPort,
    };
    use rstest::{fixture, rstest};
    use shared::error::ExternalError;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path},
    };

    use super::AtcoderProblemsClient;

    #[fixture]
    async fn server_and_client() -> (MockServer, AtcoderProblemsClient) {
        let server = MockServer::start().await;
        let client = AtcoderProblemsClient::new(server.uri().as_str());
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

        let p1 = got[0].to_owned();

        let expect1 = ApiProblem {
            id: "abc234_a".into(),
            contest_id: "abc234".into(),
            problem_index: "A".into(),
            name: "Weird Function".into(),
        };

        assert_eq!(p1, expect1);
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
    async fn fetch_problems_invalid_json(#[future(awt)] server_and_client: (MockServer, AtcoderProblemsClient)) {
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
}
