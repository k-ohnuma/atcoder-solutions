use std::sync::Arc;

use domain::ports::{
    external::{
        atcoder_problems::AtcoderProblemsPort,
        auth::{AuthenticatorPort, IdentityProviderPort},
        id::IdProviderPort,
        turnstile::TurnstileVerifierPort,
    },
    repository::{
        contest::ContestRepository,
        health::HealthCheckRepository,
        problem::ProblemRepository,
        problem::tx::ProblemTxManager,
        solution::{SolutionRepository, tx::SolutionTxManager},
        user::UserRepository,
    },
};
use infrastructure::{
    client::atcoder_problems::build_atcoder_problems_client,
    database::connect_database_with,
    ports::{
        external::{auth::FirebaseAuthenticator, id::UuidProvider, turnstile::TurnstileVerifier},
        repository::{
            contest::ContestRepositoryImpl,
            health::HealthCheckRepositoryImpl,
            problem::ProblemRepositoryImpl,
            problem::tx::ProblemTransactionManager,
            solution::{SolutionRepositoryImpl, tx::SolutionTransactionManager},
            user::UserRepositoryImpl,
        },
    },
};
use shared::config::AppConfig;

#[derive(Clone)]
pub struct Registry {
    auth_port: Arc<dyn AuthenticatorPort>,
    identity_provider: Arc<dyn IdentityProviderPort>,
    turnstile_verifier: Arc<dyn TurnstileVerifierPort>,
    atcoder_problems_port: Arc<dyn AtcoderProblemsPort>,
    health_check_repository: Arc<dyn HealthCheckRepository>,
    problem_repository: Arc<dyn ProblemRepository>,
    problem_tx_manager: Arc<dyn ProblemTxManager>,
    user_repository: Arc<dyn UserRepository>,
    id_provider: Arc<dyn IdProviderPort>,
    solution_tx_manager: Arc<dyn SolutionTxManager>,
    solution_repository: Arc<dyn SolutionRepository>,
    contest_repository: Arc<dyn ContestRepository>,
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

        let firebase_auth = Arc::new(FirebaseAuthenticator::new(&config.auth.project_id));
        let turnstile_verifier = Arc::new(TurnstileVerifier::new(
            config.turnstile.enabled,
            config.turnstile.secret_key.clone(),
            config.turnstile.siteverify_endpoint.clone(),
        ));

        let id_provider = Arc::new(UuidProvider::new());
        let solution_tx_manager = Arc::new(SolutionTransactionManager::new(pool.to_owned()));

        let solution_repository = Arc::new(SolutionRepositoryImpl::new(pool.to_owned()));
        let contest_repository = Arc::new(ContestRepositoryImpl::new(pool.to_owned()));

        Self {
            atcoder_problems_port: atcoder_problems_client,
            health_check_repository,
            problem_repository,
            problem_tx_manager,
            auth_port: firebase_auth.clone(),
            identity_provider: firebase_auth,
            turnstile_verifier,
            user_repository,
            id_provider,
            solution_tx_manager,
            solution_repository,
            contest_repository,
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
    pub fn identity_provider_port(&self) -> Arc<dyn IdentityProviderPort> {
        self.identity_provider.to_owned()
    }
    pub fn turnstile_verifier_port(&self) -> Arc<dyn TurnstileVerifierPort> {
        self.turnstile_verifier.to_owned()
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
    pub fn solution_repository(&self) -> Arc<dyn SolutionRepository> {
        self.solution_repository.to_owned()
    }
    pub fn contest_repository(&self) -> Arc<dyn ContestRepository> {
        self.contest_repository.to_owned()
    }
}
