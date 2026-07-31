use axum::extract::{Path, State};
use registry::Registry;
use usecase::{
    model::problem::get_contest_group_by_contest_series::GetContestGroupByContestSeriesInput,
    problem::get_contest_group_by_contest_series::GetContestGroupByContestSeriesUsecase,
};

use crate::{
    error::IntoApiResult,
    http::{ApiQuery, ApiResult},
    model::problem::get_contest_group_by_contest_series::{
        ContestGroupPageResponse, GetContestGroupByContestSeriesRequestParams,
    },
    response::ApiResponse,
};

pub async fn get_contest_group_by_contest_series_handler(
    State(reg): State<Registry>,
    Path(series): Path<String>,
    ApiQuery(query): ApiQuery<GetContestGroupByContestSeriesRequestParams>,
) -> ApiResult<ContestGroupPageResponse> {
    let problems_repository = reg.problem_repository();
    let usecase = GetContestGroupByContestSeriesUsecase::new(problems_repository);
    let input =
        GetContestGroupByContestSeriesInput::try_new(series, query.q, query.limit, query.offset)
            .into_api_result()?;

    let page = usecase.run(input).await.into_api_result()?;
    Ok(ApiResponse::ok(ContestGroupPageResponse::from(page)))
}
