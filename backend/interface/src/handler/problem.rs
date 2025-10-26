use axum::{
    extract::{Query, State},
    http::StatusCode,
};
use domain::model::problem::ContestSeries;
use registry::Registry;
use shared::{error::http::HttpError, response::ApiResponse};
use tracing::error;
use usecase::problem::{
    create::ImportProblemsUsecase,
    get_problems_by_contest_series::GetProblemsByContestSeriesUsecase,
};

use crate::model::problem::{
    ProblemResponse, get_problems_by_contest_series::GetProblemsByContestSeriesRequestParams,
};

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

pub async fn get_problems_by_contest_series_handler(
    State(reg): State<Registry>,
    Query(query): Query<GetProblemsByContestSeriesRequestParams>,
) -> Result<ApiResponse<Vec<ProblemResponse>>, HttpError> {
    let problems_repository = reg.problem_repository();
    let usecase = GetProblemsByContestSeriesUsecase::new(problems_repository);

    // とんでもないやつが来たらOTHERに分類される
    let series = ContestSeries::from(query.series);

    let problems = usecase.execute(series).await?;
    let resp: Vec<ProblemResponse> = problems.into_iter().map(ProblemResponse::from).collect();

    Ok(ApiResponse::ok(resp))
}
