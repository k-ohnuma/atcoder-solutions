use axum::extract::{Path, State};
use registry::Registry;
use usecase::solution::{
    create::CreateSolutionUsecase, delete::DeleteSolutionUsecase,
    get_by_problem_id::GetSolutionsByProblemIdUsecase,
    get_by_solution_id::GetSolutionBySolutionIdUsecase,
    get_by_user_name::GetSolutionsByUserNameUsecase, get_latest::GetLatestSolutionsUsecase,
    update::UpdateSolutionUsecase,
};
use uuid::Uuid;

use crate::{
    error::{HttpError, ToHttpError},
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

fn validate_limit(limit: Option<i32>) -> Result<Option<i32>, HttpError> {
    match limit {
        Some(value) if value <= 0 => Err(HttpError::BadRequest(
            "limit must be greater than 0".to_string(),
        )),
        _ => Ok(limit),
    }
}

fn validate_latest_solutions_sort(sort_by: Option<&str>) -> Result<(), HttpError> {
    if matches!(sort_by, Some(v) if v != "latest") {
        return Err(HttpError::BadRequest("sort_by must be latest".to_string()));
    }
    Ok(())
}

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
    let problem_id = problem_id.trim();
    if problem_id.is_empty() {
        return Err(HttpError::BadRequest(
            "problem_id cannot be empty".to_string(),
        ));
    }
    let sort = match req.sort_by.as_deref() {
        None | Some("latest") => usecase::model::solution::SolutionListSort::Latest,
        Some("votes") => usecase::model::solution::SolutionListSort::Votes,
        Some(_) => {
            return Err(HttpError::BadRequest(
                "sort_by must be one of: latest, votes".to_string(),
            ));
        }
    };
    let limit = validate_limit(req.limit)?;
    let solutions = uc
        .run(problem_id.to_string(), sort, limit)
        .await
        .map_err(|e| e.to_http_error())?;
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
    validate_latest_solutions_sort(req.sort_by.as_deref())?;

    let uc = GetLatestSolutionsUsecase::new(registry.solution_service());
    let limit = validate_limit(req.limit)?;
    let solutions = uc.run(limit).await.map_err(|e| e.to_http_error())?;
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
    let user_name = user_name.trim();
    if user_name.is_empty() {
        return Err(HttpError::BadRequest(
            "user_name cannot be empty".to_string(),
        ));
    }

    if matches!(req.sort_by.as_deref(), Some(v) if v != "latest" && v != "votes") {
        return Err(HttpError::BadRequest(
            "sort_by must be one of: latest, votes".to_string(),
        ));
    }

    let solutions = uc
        .run(user_name.to_string(), req.list_sort())
        .await
        .map_err(|e| e.to_http_error())?;
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

#[cfg(test)]
mod tests {
    use crate::error::HttpError;

    use super::{validate_latest_solutions_sort, validate_limit};

    #[test]
    fn validate_limit_accepts_empty_and_positive_values() {
        assert_eq!(validate_limit(None).expect("valid"), None);
        assert_eq!(validate_limit(Some(50)).expect("valid"), Some(50));
    }

    #[test]
    fn validate_limit_rejects_zero_or_negative_values() {
        assert!(matches!(
            validate_limit(Some(0)),
            Err(HttpError::BadRequest(message)) if message == "limit must be greater than 0"
        ));
        assert!(matches!(
            validate_limit(Some(-1)),
            Err(HttpError::BadRequest(message)) if message == "limit must be greater than 0"
        ));
    }

    #[test]
    fn validate_latest_solutions_sort_accepts_latest_or_empty() {
        assert!(validate_latest_solutions_sort(None).is_ok());
        assert!(validate_latest_solutions_sort(Some("latest")).is_ok());
    }

    #[test]
    fn validate_latest_solutions_sort_rejects_other_sort_values() {
        assert!(matches!(
            validate_latest_solutions_sort(Some("votes")),
            Err(HttpError::BadRequest(message)) if message == "sort_by must be latest"
        ));
    }
}
