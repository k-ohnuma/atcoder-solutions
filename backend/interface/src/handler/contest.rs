use axum::extract::{Path, State};
use registry::Registry;
use usecase::{
    contest::get_by_series::GetContestsBySeriesUseCase, model::contests::GetContestsBySeriesInput,
};

use crate::error::ToHttpError;
use crate::http::ApiResult;
use crate::model::contests::ContestResponse;
use crate::response::ApiResponse;

pub async fn get_contests_by_series_handler(
    State(reg): State<Registry>,
    Path(series): Path<String>,
) -> ApiResult<Vec<ContestResponse>> {
    let service = reg.contest_service();
    let usecase = GetContestsBySeriesUseCase::new(service);

    let input = GetContestsBySeriesInput::try_new(series).map_err(|e| e.to_http_error())?;
    let contests = usecase.run(input).await.map_err(|e| e.to_http_error())?;
    let resp: Vec<ContestResponse> = contests.into_iter().map(ContestResponse::from).collect();

    Ok(ApiResponse::ok(resp))
}
