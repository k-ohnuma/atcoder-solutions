use std::sync::Arc;

use domain::ports::{
    external::atcoder_problems::AtcoderProblemsPort, repository::health::HealthCheckRepository,
};
use infrastructure::{
    client::atcoder_problems::build_atcoder_problems_client, database::connect_database_with,
    ports::repository::health::HealthCheckRepositoryImpl,
};
use shared::config::AppConfig;

#[derive(Clone)]
pub struct Registry {
    atcoder_problems_port: Arc<dyn AtcoderProblemsPort>,
    health_check_repository: Arc<dyn HealthCheckRepository>,
}

impl Registry {
    pub fn new(config: AppConfig) -> Self {
        let atcoder_problems_client =
            Arc::new(build_atcoder_problems_client(&config.atcoder_problems));
        let pool = connect_database_with(&config.database);
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool.to_owned()));

        Self {
            atcoder_problems_port: atcoder_problems_client,
            health_check_repository,
        }
    }

    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.to_owned()
    }
}
