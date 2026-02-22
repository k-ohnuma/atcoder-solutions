use axum::extract::State;
use domain::model::problem::ContestSeries;
use registry::Registry;
use shared::{error::http::HttpError, response::ApiResponse};
use usecase::contest::get_by_series::GetContestsBySeriesUseCase;

use crate::error::ToHttpError;
use crate::http::ApiQuery;
use crate::model::contests::{
    ContestResponse, get_contests_by_series::GetContestsBySeriesRequestParams,
};

pub async fn get_contests_by_series_handler(
    State(reg): State<Registry>,
    ApiQuery(query): ApiQuery<GetContestsBySeriesRequestParams>,
) -> Result<ApiResponse<Vec<ContestResponse>>, HttpError> {
    let service = reg.contest_service();
    let usecase = GetContestsBySeriesUseCase::new(service);

    let series =
        ContestSeries::try_from(query.series).map_err(|e| HttpError::BadRequest(e.msg()))?;
    let contests = usecase.run(series).await.map_err(|e| e.to_http_error())?;
    let resp: Vec<ContestResponse> = contests.into_iter().map(ContestResponse::from).collect();

    Ok(ApiResponse::ok(resp))
}
