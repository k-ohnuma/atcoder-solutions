use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use registry::Registry;
use tracing::error;
use usecase::model::problem::{
    get_by_contest::{GetProblemByIdInput, GetProblemsByContestInput},
    get_contest_group_by_contest_series::GetContestGroupByContestSeriesInput,
};
use usecase::problem::{
    create::ImportProblemsUsecase, get_by_contest::GetProblemsByContestUsecase,
    get_by_id::GetProblemByIdUsecase,
    get_contest_group_by_contest_series::GetContestGroupByContestSeriesUsecase,
};

use crate::error::ToHttpError;
use crate::http::{ApiQuery, ApiResult};
use crate::model::problem::{
    ProblemResponse,
    get_contest_group_by_contest_series::{
        ContestGroupPageResponse, ContestProblemGroupResponse,
        GetContestGroupByContestSeriesRequestParams,
    },
};
use crate::response::ApiResponse;

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

pub async fn get_contest_group_by_contest_series_handler(
    State(reg): State<Registry>,
    Path(series): Path<String>,
    ApiQuery(query): ApiQuery<GetContestGroupByContestSeriesRequestParams>,
) -> ApiResult<ContestGroupPageResponse> {
    let problems_repository = reg.problem_repository();
    let usecase = GetContestGroupByContestSeriesUsecase::new(problems_repository);
    let input =
        GetContestGroupByContestSeriesInput::try_new(series, query.q, query.limit, query.offset)
            .map_err(|e| e.to_http_error())?;

    let page = usecase.run(input).await.map_err(|e| e.to_http_error())?;
    let groups = page
        .items
        .0
        .into_iter()
        .map(|(contest_id, problems)| ContestProblemGroupResponse {
            contest_id: contest_id.0,
            problems: problems
                .into_iter()
                .map(ProblemResponse::from)
                .collect::<Vec<_>>(),
        })
        .collect();

    Ok(ApiResponse::ok(ContestGroupPageResponse {
        groups,
        has_more: page.has_more,
        total_contest_count: page.total_contest_count,
    }))
}
