use std::{cmp::Reverse, collections::BTreeMap};

use axum::{
    extract::State,
    http::StatusCode,
};
use domain::model::problem::ContestSeries;
use registry::Registry;
use shared::{error::http::HttpError, response::ApiResponse};
use tracing::error;
use usecase::problem::{
    create::ImportProblemsUsecase, get_by_contest::GetProblemsByContestUsecase,
    get_contest_group_by_contest_series::GetContestGroupByContestSeriesUsecase,
};

use crate::error::ToHttpError;
use crate::http::ApiQuery;
use crate::model::problem::{
    ProblemResponse,
    get_contest_group_by_contest_series::GetContestGroupByContestSeriesRequestParams,
    get_problems_by_contest::GetProblemsByContestRequestParams,
};

pub async fn import_problem(reg: &Registry) -> StatusCode {
    let atcoder_problems_port = reg.atcoder_problems_port();
    let problem_tx_manager = reg.problem_tx_manager();
    let usecase = ImportProblemsUsecase::new(atcoder_problems_port, problem_tx_manager);

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
    ApiQuery(query): ApiQuery<GetProblemsByContestRequestParams>,
) -> Result<ApiResponse<Vec<ProblemResponse>>, HttpError> {
    let contest = query.contest.trim();
    if contest.is_empty() {
        return Err(HttpError::BadRequest("contest cannot be empty".to_string()));
    }

    let problems_repository = reg.problem_repository();
    let usecase = GetProblemsByContestUsecase::new(problems_repository);
    let problems = usecase.run(contest).await.map_err(|e| e.to_http_error())?;
    let resp: Vec<ProblemResponse> = problems.into_iter().map(ProblemResponse::from).collect();

    Ok(ApiResponse::ok(resp))
}

pub async fn get_contest_group_by_contest_series_handler(
    State(reg): State<Registry>,
    ApiQuery(query): ApiQuery<GetContestGroupByContestSeriesRequestParams>,
) -> Result<ApiResponse<BTreeMap<Reverse<String>, Vec<ProblemResponse>>>, HttpError> {
    let problems_repository = reg.problem_repository();
    let usecase = GetContestGroupByContestSeriesUsecase::new(problems_repository);
    let series =
        ContestSeries::try_from(query.series).map_err(|e| HttpError::BadRequest(e.msg()))?;

    let problem_map = usecase.run(series).await.map_err(|e| e.to_http_error())?;
    let resp = problem_map
        .0
        .into_iter()
        .map(|(contest_id, problems)| {
            (
                contest_id,
                problems
                    .into_iter()
                    .map(ProblemResponse::from)
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<BTreeMap<Reverse<String>, Vec<ProblemResponse>>>();

    Ok(ApiResponse::ok(resp))
}
