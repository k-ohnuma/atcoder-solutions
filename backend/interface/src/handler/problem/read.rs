use axum::extract::{Path, State};
use registry::Registry;
use usecase::{
    model::problem::get_by_contest::{GetProblemByIdInput, GetProblemsByContestInput},
    problem::{get_by_contest::GetProblemsByContestUsecase, get_by_id::GetProblemByIdUsecase},
};

use crate::{
    error::IntoApiResult, http::ApiResult, model::problem::ProblemResponse, response::ApiResponse,
};

pub async fn get_problems_by_contest_handler(
    State(reg): State<Registry>,
    Path(contest_code): Path<String>,
) -> ApiResult<Vec<ProblemResponse>> {
    let problems_repository = reg.problem_repository();
    let usecase = GetProblemsByContestUsecase::new(problems_repository);
    let input = GetProblemsByContestInput::try_new(contest_code).into_api_result()?;
    let problems = usecase.run(input).await.into_api_result()?;
    let resp: Vec<ProblemResponse> = problems.into_iter().map(ProblemResponse::from).collect();

    Ok(ApiResponse::ok(resp))
}

pub async fn get_problem_by_id_handler(
    State(reg): State<Registry>,
    Path(problem_id): Path<String>,
) -> ApiResult<ProblemResponse> {
    let problems_repository = reg.problem_repository();
    let usecase = GetProblemByIdUsecase::new(problems_repository);
    let input = GetProblemByIdInput::try_new(problem_id).into_api_result()?;
    let problem = usecase.run(input).await.into_api_result()?;

    Ok(ApiResponse::ok(ProblemResponse::from(problem)))
}
