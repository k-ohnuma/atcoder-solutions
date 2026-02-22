use std::sync::Arc;

use domain::ports::{
    external::{
        atcoder_problems::AtcoderProblemsPort, auth::AuthenticatorPort, id::IdProviderPort,
    },
    repository::{
        health::HealthCheckRepository, problem::ProblemRepository, problem::tx::ProblemTxManager,
        solution::tx::SolutionTxManager, user::UserRepository,
    },
};
use infrastructure::{
    client::atcoder_problems::build_atcoder_problems_client,
    database::connect_database_with,
    ports::{
        external::{auth::FirebaseAuthenticator, id::UuidProvider},
        repository::{
            health::HealthCheckRepositoryImpl, problem::ProblemRepositoryImpl,
            problem::tx::ProblemTransactionManager, solution::tx::SolutionTransactionManager,
            user::UserRepositoryImpl,
        },
        service::{contests::ContestServiceImpl, solution::SolutionServiceImpl},
    },
};
use shared::config::AppConfig;
use usecase::service::{contest::ContestService, solution::SolutionService};

#[derive(Clone)]
pub struct Registry {
    auth_port: Arc<dyn AuthenticatorPort>,
    atcoder_problems_port: Arc<dyn AtcoderProblemsPort>,
    health_check_repository: Arc<dyn HealthCheckRepository>,
    problem_repository: Arc<dyn ProblemRepository>,
    problem_tx_manager: Arc<dyn ProblemTxManager>,
    user_repository: Arc<dyn UserRepository>,
    id_provider: Arc<dyn IdProviderPort>,
    solution_tx_manager: Arc<dyn SolutionTxManager>,
    solution_service: Arc<dyn SolutionService>,
    contest_service: Arc<dyn ContestService>,
}

impl Registry {
    pub fn new(config: AppConfig) -> Self {
        let atcoder_problems_client =
            Arc::new(build_atcoder_problems_client(&config.atcoder_problems));
        let pool = connect_database_with(&config.database);
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool.to_owned()));
        let problem_repository = Arc::new(ProblemRepositoryImpl::new(pool.to_owned()));
        let problem_tx_manager = Arc::new(ProblemTransactionManager::new(pool.to_owned()));
        let user_repository = Arc::new(UserRepositoryImpl::new(pool.to_owned()));

        let authenticator = Arc::new(FirebaseAuthenticator::new(&config.auth.project_id));

        let id_provider = Arc::new(UuidProvider::new());
        let solution_tx_manager = Arc::new(SolutionTransactionManager::new(pool.to_owned()));

        let solution_service = Arc::new(SolutionServiceImpl::new(pool.to_owned()));
        let contest_service = Arc::new(ContestServiceImpl::new(pool.to_owned()));

        Self {
            atcoder_problems_port: atcoder_problems_client,
            health_check_repository,
            problem_repository,
            problem_tx_manager,
            auth_port: authenticator,
            user_repository,
            id_provider,
            solution_tx_manager,
            solution_service,
            contest_service,
        }
    }

    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.to_owned()
    }
    pub fn problem_repository(&self) -> Arc<dyn ProblemRepository> {
        self.problem_repository.to_owned()
    }
    pub fn problem_tx_manager(&self) -> Arc<dyn ProblemTxManager> {
        self.problem_tx_manager.to_owned()
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
    pub fn id_provider_port(&self) -> Arc<dyn IdProviderPort> {
        self.id_provider.to_owned()
    }
    pub fn solution_tx_manager(&self) -> Arc<dyn SolutionTxManager> {
        self.solution_tx_manager.to_owned()
    }
    pub fn solution_service(&self) -> Arc<dyn SolutionService> {
        self.solution_service.to_owned()
    }
    pub fn contest_service(&self) -> Arc<dyn ContestService> {
        self.contest_service.to_owned()
    }
}
