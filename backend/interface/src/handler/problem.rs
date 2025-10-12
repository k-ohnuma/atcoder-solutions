use axum::http::StatusCode;
use registry::Registry;
use tracing::error;
use usecase::problem::ImportProblemsUsecase;

pub async fn import_problem(reg: &Registry) -> StatusCode {
    let atcoder_problems_port = reg.atcoder_problems_port();
    let problems_repository = reg.problem_repository();
    let usecase = ImportProblemsUsecase::new(atcoder_problems_port, problems_repository);

    match usecase.execute().await {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            error!(error = ?e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
