use axum::{
    Json,
    extract::{Path, State},
};
use registry::Registry;
use shared::{error::http::HttpError, response::ApiResponse};
use usecase::model::solution::create_comment::CreateCommentInput;
use usecase::solution::{
    create::CreateSolutionUsecase, create_comment::CreateCommentUsecase,
    delete::DeleteSolutionUsecase, delete_comment::DeleteCommentUsecase,
    get_by_problem_id::GetSolutionsByProblemIdUsecase,
    get_by_solution_id::GetSolutionBySolutionIdUsecase,
    get_by_user_name::GetSolutionsByUserNameUsecase,
    get_comments_by_solution_id::GetCommentsBySolutionIdUsecase,
    get_latest::GetLatestSolutionsUsecase, get_my_vote_status::GetMyVoteStatusUsecase,
    get_votes_count::GetSolutionVotesCountUsecase, unvote::UnvoteSolutionUsecase,
    update::UpdateSolutionUsecase, update_comment::UpdateCommentUsecase, vote::VoteSolutionUsecase,
};

use crate::{
    error::ToHttpError,
    http::{ApiJson, ApiQuery, AuthUser},
    model::solution::{
        create_comment::{CreateCommentRequest, CreateCommentResponse},
        create_solution::{CreateSolutionRequest, CreateSolutionResponse, from_req_for_input},
        delete_comment::DeleteCommentResponse,
        delete_solution::DeleteSolutionResponse,
        get_comments_by_solution_id::GetCommentsBySolutionIdResponse,
        get_latest_solutions::{GetLatestSolutionsRequest, GetLatestSolutionsResponse},
        get_my_vote_status::GetMyVoteStatusResponse,
        get_solution_by_solution_id::GetSolutionBySolutionIdResponse,
        get_solution_votes_count::GetSolutionVotesCountResponse,
        get_solutions_by_problem_id::{
            GetSolutionsByProblemIdRequest, GetSolutionsByProblemIdResponse,
        },
        get_solutions_by_user_name::{
            GetSolutionsByUserNameRequest, GetSolutionsByUserNameResponse,
        },
        unvote_solution::UnvoteSolutionResponse,
        update_comment::{
            UpdateCommentRequest, UpdateCommentResponse,
            from_req_for_input as from_req_for_update_comment,
        },
        update_solution::{
            UpdateSolutionRequest, UpdateSolutionResponse,
            from_req_for_input as from_req_for_update_solution,
        },
        vote_solution::VoteSolutionResponse,
    },
};
use uuid::Uuid;

fn validate_size(size: Option<i32>) -> Result<Option<i32>, HttpError> {
    match size {
        Some(value) if value <= 0 => Err(HttpError::BadRequest(
            "size must be greater than 0".to_string(),
        )),
        _ => Ok(size),
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
    ApiJson(req): ApiJson<CreateSolutionRequest>,
) -> Result<Json<ApiResponse<CreateSolutionResponse>>, HttpError> {
    let user_id = user.uid;
    let repo = CreateSolutionUsecase::new(
        registry.id_provider_port(),
        registry.solution_tx_manager(),
        registry.solution_service(),
    );
    let input = from_req_for_input(user_id, req);
    let res = repo.run(input).await.map_err(|e| e.to_http_error())?;

    Ok(Json(ApiResponse::ok(res.into())))
}

pub async fn update_solution_handler(
    State(registry): State<Registry>,
    Path(solution_id): Path<Uuid>,
    AuthUser(user): AuthUser,
    ApiJson(req): ApiJson<UpdateSolutionRequest>,
) -> Result<Json<ApiResponse<UpdateSolutionResponse>>, HttpError> {
    let uc =
        UpdateSolutionUsecase::new(registry.solution_tx_manager(), registry.solution_service());
    let input = from_req_for_update_solution(user.uid, solution_id, req);
    let updated_id = uc.run(input).await.map_err(|e| e.to_http_error())?;
    Ok(Json(ApiResponse::ok(updated_id.into())))
}

pub async fn delete_solution_handler(
    State(registry): State<Registry>,
    Path(solution_id): Path<Uuid>,
    AuthUser(user): AuthUser,
) -> Result<Json<ApiResponse<DeleteSolutionResponse>>, HttpError> {
    let uc =
        DeleteSolutionUsecase::new(registry.solution_tx_manager(), registry.solution_service());
    let deleted_id = uc
        .run(user.uid, solution_id)
        .await
        .map_err(|e| e.to_http_error())?;
    Ok(Json(ApiResponse::ok(deleted_id.into())))
}
pub async fn get_solutions_by_problems_id_handler(
    State(registry): State<Registry>,
    ApiQuery(req): ApiQuery<GetSolutionsByProblemIdRequest>,
) -> Result<Json<ApiResponse<Vec<GetSolutionsByProblemIdResponse>>>, HttpError> {
    let uc = GetSolutionsByProblemIdUsecase::new(registry.solution_service());
    let problem_id = req.problem_id.trim();
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
    let size = validate_size(req.size)?;
    let solutions = uc
        .run(problem_id.to_string(), sort, size)
        .await
        .map_err(|e| e.to_http_error())?;
    let ret: Vec<_> = solutions
        .into_iter()
        .map(GetSolutionsByProblemIdResponse::from)
        .collect();
    Ok(Json(ApiResponse::ok(ret)))
}

pub async fn get_latest_solutions_handler(
    State(registry): State<Registry>,
    ApiQuery(req): ApiQuery<GetLatestSolutionsRequest>,
) -> Result<Json<ApiResponse<Vec<GetLatestSolutionsResponse>>>, HttpError> {
    validate_latest_solutions_sort(req.sort_by.as_deref())?;

    let uc = GetLatestSolutionsUsecase::new(registry.solution_service());
    let size = validate_size(req.size)?;
    let solutions = uc.run(size).await.map_err(|e| e.to_http_error())?;
    let ret: Vec<_> = solutions
        .into_iter()
        .map(GetLatestSolutionsResponse::from)
        .collect();
    Ok(Json(ApiResponse::ok(ret)))
}

pub async fn get_solutions_by_user_name_handler(
    State(registry): State<Registry>,
    ApiQuery(req): ApiQuery<GetSolutionsByUserNameRequest>,
) -> Result<Json<ApiResponse<Vec<GetSolutionsByUserNameResponse>>>, HttpError> {
    let uc = GetSolutionsByUserNameUsecase::new(registry.solution_service());
    let user_name = req.user_name.trim();
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
    Ok(Json(ApiResponse::ok(ret)))
}

pub async fn get_solution_by_solution_id_handler(
    State(registry): State<Registry>,
    Path(solution_id): Path<Uuid>,
) -> Result<Json<ApiResponse<GetSolutionBySolutionIdResponse>>, HttpError> {
    let uc = GetSolutionBySolutionIdUsecase::new(registry.solution_service());
    let solution = uc.run(solution_id).await.map_err(|e| e.to_http_error())?;
    Ok(Json(ApiResponse::ok(solution.into())))
}

pub async fn vote_solution_handler(
    State(registry): State<Registry>,
    Path(solution_id): Path<Uuid>,
    AuthUser(user): AuthUser,
) -> Result<Json<ApiResponse<VoteSolutionResponse>>, HttpError> {
    let uc = VoteSolutionUsecase::new(registry.solution_tx_manager(), registry.solution_service());
    uc.run(user.uid, solution_id)
        .await
        .map_err(|e| e.to_http_error())?;
    Ok(Json(ApiResponse::ok(VoteSolutionResponse::liked(
        solution_id,
    ))))
}

pub async fn unvote_solution_handler(
    State(registry): State<Registry>,
    Path(solution_id): Path<Uuid>,
    AuthUser(user): AuthUser,
) -> Result<Json<ApiResponse<UnvoteSolutionResponse>>, HttpError> {
    let uc =
        UnvoteSolutionUsecase::new(registry.solution_tx_manager(), registry.solution_service());
    uc.run(user.uid, solution_id)
        .await
        .map_err(|e| e.to_http_error())?;
    Ok(Json(ApiResponse::ok(UnvoteSolutionResponse::unliked(
        solution_id,
    ))))
}

pub async fn get_solution_votes_count_handler(
    State(registry): State<Registry>,
    Path(solution_id): Path<Uuid>,
) -> Result<Json<ApiResponse<GetSolutionVotesCountResponse>>, HttpError> {
    let uc = GetSolutionVotesCountUsecase::new(registry.solution_service());
    let count = uc.run(solution_id).await.map_err(|e| e.to_http_error())?;
    Ok(Json(ApiResponse::ok(GetSolutionVotesCountResponse::new(
        solution_id,
        count,
    ))))
}

pub async fn get_my_vote_status_handler(
    State(registry): State<Registry>,
    Path(solution_id): Path<Uuid>,
    AuthUser(user): AuthUser,
) -> Result<Json<ApiResponse<GetMyVoteStatusResponse>>, HttpError> {
    let uc = GetMyVoteStatusUsecase::new(registry.solution_service());
    let liked = uc
        .run(user.uid, solution_id)
        .await
        .map_err(|e| e.to_http_error())?;
    Ok(Json(ApiResponse::ok(GetMyVoteStatusResponse::new(
        solution_id,
        liked,
    ))))
}

pub async fn create_comment_handler(
    State(registry): State<Registry>,
    Path(solution_id): Path<Uuid>,
    AuthUser(user): AuthUser,
    ApiJson(req): ApiJson<CreateCommentRequest>,
) -> Result<Json<ApiResponse<CreateCommentResponse>>, HttpError> {
    let uc = CreateCommentUsecase::new(registry.solution_tx_manager(), registry.solution_service());
    let input = CreateCommentInput {
        user_id: user.uid,
        solution_id,
        body_md: req.body_md,
    };
    let created = uc.run(input).await.map_err(|e| e.to_http_error())?;
    Ok(Json(ApiResponse::ok(CreateCommentResponse::from(created))))
}

pub async fn update_comment_handler(
    State(registry): State<Registry>,
    Path(comment_id): Path<Uuid>,
    AuthUser(user): AuthUser,
    ApiJson(req): ApiJson<UpdateCommentRequest>,
) -> Result<Json<ApiResponse<UpdateCommentResponse>>, HttpError> {
    let uc = UpdateCommentUsecase::new(registry.solution_tx_manager(), registry.solution_service());
    let input = from_req_for_update_comment(user.uid, comment_id, req);
    let updated = uc.run(input).await.map_err(|e| e.to_http_error())?;
    Ok(Json(ApiResponse::ok(UpdateCommentResponse::from(updated))))
}

pub async fn delete_comment_handler(
    State(registry): State<Registry>,
    Path(comment_id): Path<Uuid>,
    AuthUser(user): AuthUser,
) -> Result<Json<ApiResponse<DeleteCommentResponse>>, HttpError> {
    let uc = DeleteCommentUsecase::new(registry.solution_tx_manager(), registry.solution_service());
    let deleted_id = uc
        .run(user.uid, comment_id)
        .await
        .map_err(|e| e.to_http_error())?;
    Ok(Json(ApiResponse::ok(deleted_id.into())))
}

pub async fn get_comments_by_solution_id_handler(
    State(registry): State<Registry>,
    Path(solution_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<GetCommentsBySolutionIdResponse>>>, HttpError> {
    let uc = GetCommentsBySolutionIdUsecase::new(registry.solution_service());
    let comments = uc.run(solution_id).await.map_err(|e| e.to_http_error())?;
    let ret: Vec<_> = comments
        .into_iter()
        .map(GetCommentsBySolutionIdResponse::from)
        .collect();
    Ok(Json(ApiResponse::ok(ret)))
}

#[cfg(test)]
mod tests {
    use shared::error::http::HttpError;

    use super::{validate_latest_solutions_sort, validate_size};

    #[test]
    fn validate_size_accepts_empty_and_positive_values() {
        assert_eq!(validate_size(None).expect("valid"), None);
        assert_eq!(validate_size(Some(50)).expect("valid"), Some(50));
    }

    #[test]
    fn validate_size_rejects_zero_or_negative_values() {
        assert!(matches!(
            validate_size(Some(0)),
            Err(HttpError::BadRequest(message)) if message == "size must be greater than 0"
        ));
        assert!(matches!(
            validate_size(Some(-1)),
            Err(HttpError::BadRequest(message)) if message == "size must be greater than 0"
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
