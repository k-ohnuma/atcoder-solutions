use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use domain::model::problem::ContestSeries;
use registry::Registry;
use shared::{error::http::HttpError, response::ApiResponse};
use tracing::error;
use usecase::problem::{
    create::ImportProblemsUsecase, get_by_contest::GetProblemsByContestUsecase,
    get_by_id::GetProblemByIdUsecase,
    get_contest_group_by_contest_series::GetContestGroupByContestSeriesUsecase,
};

use crate::error::ToHttpError;
use crate::http::ApiQuery;
use crate::model::problem::{
    ProblemResponse,
    get_contest_group_by_contest_series::{
        ContestGroupPageResponse, GetContestGroupByContestSeriesRequestParams,
    },
    get_problems_by_contest::GetProblemsByContestRequestParams,
};

const MAX_CONTEST_GROUP_QUERY_LENGTH: usize = 100;
const MAX_CONTEST_GROUP_OFFSET: usize = 5_000;

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

pub async fn get_problem_by_id_handler(
    State(reg): State<Registry>,
    Path(problem_id): Path<String>,
) -> Result<ApiResponse<ProblemResponse>, HttpError> {
    let problem_id = problem_id.trim();
    if problem_id.is_empty() {
        return Err(HttpError::BadRequest(
            "problemId cannot be empty".to_string(),
        ));
    }

    let problems_repository = reg.problem_repository();
    let usecase = GetProblemByIdUsecase::new(problems_repository);
    let problem = usecase
        .run(problem_id)
        .await
        .map_err(|e| e.to_http_error())?;

    Ok(ApiResponse::ok(ProblemResponse::from(problem)))
}

pub async fn get_contest_group_by_contest_series_handler(
    State(reg): State<Registry>,
    ApiQuery(query): ApiQuery<GetContestGroupByContestSeriesRequestParams>,
) -> Result<ApiResponse<ContestGroupPageResponse>, HttpError> {
    let problems_repository = reg.problem_repository();
    let usecase = GetContestGroupByContestSeriesUsecase::new(problems_repository);
    let series =
        ContestSeries::try_from(query.series).map_err(|e| HttpError::BadRequest(e.msg()))?;
    if query
        .q
        .as_ref()
        .is_some_and(|q| q.chars().count() > MAX_CONTEST_GROUP_QUERY_LENGTH)
    {
        return Err(HttpError::BadRequest(format!(
            "q must be at most {MAX_CONTEST_GROUP_QUERY_LENGTH} characters"
        )));
    }
    if query
        .offset
        .is_some_and(|offset| offset > MAX_CONTEST_GROUP_OFFSET)
    {
        return Err(HttpError::BadRequest(format!(
            "offset must be at most {MAX_CONTEST_GROUP_OFFSET}"
        )));
    }

    let page = usecase
        .run(series, query.q, query.limit, query.offset)
        .await
        .map_err(|e| e.to_http_error())?;
    let items = page
        .items
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
        .collect();

    Ok(ApiResponse::ok(ContestGroupPageResponse {
        items,
        has_more: page.has_more,
        total_contest_count: page.total_contest_count,
    }))
}
