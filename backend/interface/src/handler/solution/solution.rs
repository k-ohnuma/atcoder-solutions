use axum::extract::{Path, State};
use registry::Registry;
use usecase::model::solution::list::{
    GetLatestSolutionsInput, GetSolutionsByProblemIdInput, GetSolutionsByUserNameInput,
};
use usecase::solution::{
    create::CreateSolutionUsecase, delete::DeleteSolutionUsecase,
    get_by_problem_id::GetSolutionsByProblemIdUsecase,
    get_by_solution_id::GetSolutionBySolutionIdUsecase,
    get_by_user_name::GetSolutionsByUserNameUsecase, get_latest::GetLatestSolutionsUsecase,
    update::UpdateSolutionUsecase,
};
use uuid::Uuid;

use crate::{
    error::ToHttpError,
    http::{ApiJson, ApiQuery, ApiResult, AuthUser, TurnstileGuard},
    model::solution::{
        create_solution::{CreateSolutionRequest, CreateSolutionResponse, from_req_for_input},
        delete_solution::DeleteSolutionResponse,
        get_latest_solutions::{GetLatestSolutionsRequest, GetLatestSolutionsResponse},
        get_solution_by_solution_id::GetSolutionBySolutionIdResponse,
        get_solutions_by_problem_id::{
            GetSolutionsByProblemIdRequest, GetSolutionsByProblemIdResponse,
        },
        get_solutions_by_user_name::{
            GetSolutionsByUserNameRequest, GetSolutionsByUserNameResponse,
        },
        update_solution::{
            UpdateSolutionRequest, UpdateSolutionResponse,
            from_req_for_input as from_req_for_update_solution,
        },
    },
    response::ApiResponse,
};

pub async fn create_solution_handler(
    State(registry): State<Registry>,
    AuthUser(user): AuthUser,
    _turnstile_guard: TurnstileGuard,
    ApiJson(req): ApiJson<CreateSolutionRequest>,
) -> ApiResult<CreateSolutionResponse> {
    let user_id = user.uid;
    let repo = CreateSolutionUsecase::new(
        registry.id_provider_port(),
        registry.solution_tx_manager(),
        registry.solution_service(),
    );
    let input = from_req_for_input(user_id, req);
    let res = repo.run(input).await.map_err(|e| e.to_http_error())?;

    Ok(ApiResponse::ok(res.into()))
}

pub async fn update_solution_handler(
    State(registry): State<Registry>,
    Path(solution_id): Path<Uuid>,
    AuthUser(user): AuthUser,
    ApiJson(req): ApiJson<UpdateSolutionRequest>,
) -> ApiResult<UpdateSolutionResponse> {
    let uc =
        UpdateSolutionUsecase::new(registry.solution_tx_manager(), registry.solution_service());
    let input = from_req_for_update_solution(user.uid, solution_id, req);
    let updated_id = uc.run(input).await.map_err(|e| e.to_http_error())?;
    Ok(ApiResponse::ok(updated_id.into()))
}

pub async fn delete_solution_handler(
    State(registry): State<Registry>,
    Path(solution_id): Path<Uuid>,
    AuthUser(user): AuthUser,
) -> ApiResult<DeleteSolutionResponse> {
    let uc =
        DeleteSolutionUsecase::new(registry.solution_tx_manager(), registry.solution_service());
    let deleted_id = uc
        .run(user.uid, solution_id)
        .await
        .map_err(|e| e.to_http_error())?;
    Ok(ApiResponse::ok(deleted_id.into()))
}

pub async fn get_solutions_by_problems_id_handler(
    State(registry): State<Registry>,
    Path(problem_id): Path<String>,
    ApiQuery(req): ApiQuery<GetSolutionsByProblemIdRequest>,
) -> ApiResult<Vec<GetSolutionsByProblemIdResponse>> {
    let uc = GetSolutionsByProblemIdUsecase::new(registry.solution_service());
    let input = GetSolutionsByProblemIdInput::try_new(problem_id, req.sort_by, req.limit)
        .map_err(|e| e.to_http_error())?;
    let solutions = uc.run(input).await.map_err(|e| e.to_http_error())?;
    let ret: Vec<_> = solutions
        .into_iter()
        .map(GetSolutionsByProblemIdResponse::from)
        .collect();
    Ok(ApiResponse::ok(ret))
}

pub async fn get_latest_solutions_handler(
    State(registry): State<Registry>,
    ApiQuery(req): ApiQuery<GetLatestSolutionsRequest>,
) -> ApiResult<Vec<GetLatestSolutionsResponse>> {
    let uc = GetLatestSolutionsUsecase::new(registry.solution_service());
    let input =
        GetLatestSolutionsInput::try_new(req.sort_by, req.limit).map_err(|e| e.to_http_error())?;
    let solutions = uc.run(input).await.map_err(|e| e.to_http_error())?;
    let ret: Vec<_> = solutions
        .into_iter()
        .map(GetLatestSolutionsResponse::from)
        .collect();
    Ok(ApiResponse::ok(ret))
}

pub async fn get_solutions_by_user_name_handler(
    State(registry): State<Registry>,
    Path(user_name): Path<String>,
    ApiQuery(req): ApiQuery<GetSolutionsByUserNameRequest>,
) -> ApiResult<Vec<GetSolutionsByUserNameResponse>> {
    let uc = GetSolutionsByUserNameUsecase::new(registry.solution_service());
    let input = GetSolutionsByUserNameInput::try_new(user_name, req.sort_by)
        .map_err(|e| e.to_http_error())?;

    let solutions = uc.run(input).await.map_err(|e| e.to_http_error())?;
    let ret: Vec<_> = solutions
        .into_iter()
        .map(GetSolutionsByUserNameResponse::from)
        .collect();
    Ok(ApiResponse::ok(ret))
}

pub async fn get_solution_by_solution_id_handler(
    State(registry): State<Registry>,
    Path(solution_id): Path<Uuid>,
) -> ApiResult<GetSolutionBySolutionIdResponse> {
    let uc = GetSolutionBySolutionIdUsecase::new(registry.solution_service());
    let solution = uc.run(solution_id).await.map_err(|e| e.to_http_error())?;
    Ok(ApiResponse::ok(solution.into()))
}
