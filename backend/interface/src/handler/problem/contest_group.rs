use axum::extract::{Path, State};
use registry::Registry;
use usecase::{
    model::problem::get_contest_group_by_contest_series::GetContestGroupByContestSeriesInput,
    problem::get_contest_group_by_contest_series::GetContestGroupByContestSeriesUsecase,
};

use crate::{
    error::ToHttpError,
    http::{ApiQuery, ApiResult},
    model::problem::{
        ProblemResponse,
        get_contest_group_by_contest_series::{
            ContestGroupPageResponse, ContestProblemGroupResponse,
            GetContestGroupByContestSeriesRequestParams,
        },
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
