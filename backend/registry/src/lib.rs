use std::sync::Arc;

use domain::ports::{
    external::{atcoder_problems::AtcoderProblemsPort, auth::AuthenticatorPort},
    repository::{health::HealthCheckRepository, problem::ProblemRepository, user::UserRepository},
};
use infrastructure::{
    client::atcoder_problems::build_atcoder_problems_client,
    database::connect_database_with,
    ports::{
        external::auth::FirebaseAuthenticator,
        repository::{health::HealthCheckRepositoryImpl, problem::ProblemRepositoryImpl, user::UserRepositoryImpl},
    },
};
use shared::config::AppConfig;

#[derive(Clone)]
pub struct Registry {
    auth_port: Arc<dyn AuthenticatorPort>,
    atcoder_problems_port: Arc<dyn AtcoderProblemsPort>,
    health_check_repository: Arc<dyn HealthCheckRepository>,
    problem_repository: Arc<dyn ProblemRepository>,
    user_repository: Arc<dyn UserRepository>
}

impl Registry {
    pub fn new(config: AppConfig) -> Self {
        let atcoder_problems_client =
            Arc::new(build_atcoder_problems_client(&config.atcoder_problems));
        let pool = connect_database_with(&config.database);
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool.to_owned()));
        let problem_repository = Arc::new(ProblemRepositoryImpl::new(pool.to_owned()));
        let user_repository = Arc::new(UserRepositoryImpl::new(pool.to_owned()));

        let authenticator = Arc::new(FirebaseAuthenticator::new(&config.auth.project_id));

        Self {
            atcoder_problems_port: atcoder_problems_client,
            health_check_repository,
            problem_repository,
            auth_port: authenticator,
            user_repository
        }
    }

    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.to_owned()
    }
    pub fn problem_repository(&self) -> Arc<dyn ProblemRepository> {
        self.problem_repository.to_owned()
    }
    pub fn atcoder_problems_port(&self) -> Arc<dyn AtcoderProblemsPort> {
        self.atcoder_problems_port.to_owned()
    }
    pub fn auth_port(&self) -> Arc<dyn AuthenticatorPort> {
        self.auth_port.to_owned()
    }
    pub fn user_repository(&self) -> Arc<dyn UserRepository> {
        self.user_repository.to_owned()
    }
}
