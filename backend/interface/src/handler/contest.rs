use axum::extract::{Path, State};
use registry::Registry;
use usecase::{
    contest::get_by_series::GetContestsBySeriesUsecase, model::contests::GetContestsBySeriesInput,
};

use crate::error::IntoApiResult;
use crate::http::ApiResult;
use crate::model::contests::ContestResponse;
use crate::response::ApiResponse;

pub async fn get_contests_by_series_handler(
    State(reg): State<Registry>,
    Path(series): Path<String>,
) -> ApiResult<Vec<ContestResponse>> {
    let service = reg.contest_repository();
    let usecase = GetContestsBySeriesUsecase::new(service);

    let input = GetContestsBySeriesInput::try_new(series).into_api_result()?;
    let contests = usecase.run(input).await.into_api_result()?;
    let resp: Vec<ContestResponse> = contests.into_iter().map(ContestResponse::from).collect();

    Ok(ApiResponse::ok(resp))
}
