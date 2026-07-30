use axum::extract::{Path, State};
use domain::model::problem::ContestSeries;
use registry::Registry;
use usecase::contest::get_by_series::GetContestsBySeriesUseCase;

use crate::error::{HttpError, ToHttpError};
use crate::http::ApiResult;
use crate::model::contests::ContestResponse;
use crate::response::ApiResponse;

pub async fn get_contests_by_series_handler(
    State(reg): State<Registry>,
    Path(series): Path<String>,
) -> ApiResult<Vec<ContestResponse>> {
    let service = reg.contest_service();
    let usecase = GetContestsBySeriesUseCase::new(service);

    let series = ContestSeries::try_from(series).map_err(|e| HttpError::BadRequest(e.msg()))?;
    let contests = usecase.run(series).await.map_err(|e| e.to_http_error())?;
    let resp: Vec<ContestResponse> = contests.into_iter().map(ContestResponse::from).collect();

    Ok(ApiResponse::ok(resp))
}
