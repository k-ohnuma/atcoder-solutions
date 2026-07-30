use axum::extract::{Path, State};
use registry::Registry;
use usecase::{
    model::problem::get_by_contest::{GetProblemByIdInput, GetProblemsByContestInput},
    problem::{get_by_contest::GetProblemsByContestUsecase, get_by_id::GetProblemByIdUsecase},
};

use crate::{
    error::ToHttpError, http::ApiResult, model::problem::ProblemResponse, response::ApiResponse,
};

pub async fn get_problems_by_contest_handler(
    State(reg): State<Registry>,
    Path(contest_code): Path<String>,
) -> ApiResult<Vec<ProblemResponse>> {
    let problems_repository = reg.problem_repository();
    let usecase = GetProblemsByContestUsecase::new(problems_repository);
    let input = GetProblemsByContestInput::try_new(contest_code).map_err(|e| e.to_http_error())?;
    let problems = usecase.run(input).await.map_err(|e| e.to_http_error())?;
    let resp: Vec<ProblemResponse> = problems.into_iter().map(ProblemResponse::from).collect();

    Ok(ApiResponse::ok(resp))
}

pub async fn get_problem_by_id_handler(
    State(reg): State<Registry>,
    Path(problem_id): Path<String>,
) -> ApiResult<ProblemResponse> {
    let problems_repository = reg.problem_repository();
    let usecase = GetProblemByIdUsecase::new(problems_repository);
    let input = GetProblemByIdInput::try_new(problem_id).map_err(|e| e.to_http_error())?;
    let problem = usecase.run(input).await.map_err(|e| e.to_http_error())?;

    Ok(ApiResponse::ok(ProblemResponse::from(problem)))
}
