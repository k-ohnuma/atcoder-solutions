use axum::http::StatusCode;
use registry::Registry;
use tracing::error;
use usecase::problem::create::ImportProblemsUsecase;

pub async fn import_problem(reg: &Registry) -> StatusCode {
    let atcoder_problems_port = reg.atcoder_problems_port();
    let problem_repository = reg.problem_repository();
    let problem_tx_manager = reg.problem_tx_manager();
    let usecase = ImportProblemsUsecase::new(
        atcoder_problems_port,
        problem_repository,
        problem_tx_manager,
    );

    match usecase.run().await {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            error!(error = ?e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
