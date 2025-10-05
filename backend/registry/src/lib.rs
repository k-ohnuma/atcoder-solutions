use std::sync::Arc;

use domain::ports::external::atcoder_problems::AtcoderProblemsPort;
use infrastructure::{client::atcoder_problems::build_atcoder_problems_client, database::connect_database_with};
use shared::config::AppConfig;

pub struct Registroy {
    atcoderProblemsClient: Arc<dyn AtcoderProblemsPort>
}

impl Registroy {
    pub fn new(config: AppConfig) -> Self {
        let atcoder_problems_client = Arc::new(build_atcoder_problems_client(&config.atcoder_problems));
        let pool = connect_database_with(&config.database);
        

        Self {
            atcoderProblemsClient: atcoder_problems_client
        }
    }
}
