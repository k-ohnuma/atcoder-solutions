use axum::extract::{Query, State};
use domain::model::problem::ContestSeries;
use registry::Registry;
use shared::{error::http::HttpError, response::ApiResponse};
use usecase::contest::get_by_series::GetContestsBySeriesUseCase;

use crate::model::contests::{
    ContestResponse, get_contests_by_series::GetContestsBySeriesRequestParams,
};

pub async fn get_contests_by_series_handler(
    State(reg): State<Registry>,
    Query(query): Query<GetContestsBySeriesRequestParams>,
) -> Result<ApiResponse<Vec<ContestResponse>>, HttpError> {
    let service = reg.contest_service();
    let usecase = GetContestsBySeriesUseCase::new(service);

    let series =
        ContestSeries::try_from(query.series).map_err(|e| HttpError::BadRequest(e.msg()))?;
    let contests = usecase.run(series).await?;
    let resp: Vec<ContestResponse> = contests.into_iter().map(ContestResponse::from).collect();

    Ok(ApiResponse::ok(resp))
}
