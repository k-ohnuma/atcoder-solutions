use shared::config::AtcoderProblemsConfig;

use crate::ports::external::atcoder_problems::AtcoderProblemsClient;

pub fn build_atcoder_problems_client(cfg: &AtcoderProblemsConfig) -> AtcoderProblemsClient {
    AtcoderProblemsClient::new(cfg.base_endpoint.as_str())
}
