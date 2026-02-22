use axum::{
    extract::State,
    Json,
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
    get_my_vote_status::GetMyVoteStatusUsecase, get_votes_count::GetSolutionVotesCountUsecase,
    unvote::UnvoteSolutionUsecase, update::UpdateSolutionUsecase,
    update_comment::UpdateCommentUsecase, vote::VoteSolutionUsecase,
};

use crate::{
    error::ToHttpError,
    http::{ApiJson, ApiQuery, AuthUser},
    model::solution::{
        create_comment::{CreateCommentRequest, CreateCommentResponse},
        create_solution::{CreateSolutionRequest, CreateSolutionResponse, from_req_for_input},
        delete_comment::{DeleteCommentRequest, DeleteCommentResponse},
        delete_solution::{DeleteSolutionRequest, DeleteSolutionResponse},
        get_comments_by_solution_id::{
            GetCommentsBySolutionIdRequest, GetCommentsBySolutionIdResponse,
        },
        get_my_vote_status::{GetMyVoteStatusRequest, GetMyVoteStatusResponse},
        get_solution_by_solution_id::{
            GetSolutionBySolutionIdRequest, GetSolutionBySolutionIdResponse,
        },
        get_solution_votes_count::{GetSolutionVotesCountRequest, GetSolutionVotesCountResponse},
        get_solutions_by_problem_id::{
            GetSolutionsByProblemIdRequest, GetSolutionsByProblemIdResponse,
        },
        get_solutions_by_user_name::{
            GetSolutionsByUserNameRequest, GetSolutionsByUserNameResponse,
        },
        unvote_solution::{UnvoteSolutionRequest, UnvoteSolutionResponse},
        update_comment::{
            UpdateCommentRequest, UpdateCommentResponse,
            from_req_for_input as from_req_for_update_comment,
        },
        update_solution::{
            UpdateSolutionRequest, UpdateSolutionResponse,
            from_req_for_input as from_req_for_update_solution,
        },
        vote_solution::{VoteSolutionRequest, VoteSolutionResponse},
    },
};

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
    AuthUser(user): AuthUser,
    ApiJson(req): ApiJson<UpdateSolutionRequest>,
) -> Result<Json<ApiResponse<UpdateSolutionResponse>>, HttpError> {
    let uc =
        UpdateSolutionUsecase::new(registry.solution_tx_manager(), registry.solution_service());
    let input = from_req_for_update_solution(user.uid, req);
    let updated_id = uc.run(input).await.map_err(|e| e.to_http_error())?;
    Ok(Json(ApiResponse::ok(updated_id.into())))
}

pub async fn delete_solution_handler(
    State(registry): State<Registry>,
    AuthUser(user): AuthUser,
    ApiQuery(req): ApiQuery<DeleteSolutionRequest>,
) -> Result<Json<ApiResponse<DeleteSolutionResponse>>, HttpError> {
    let uc =
        DeleteSolutionUsecase::new(registry.solution_tx_manager(), registry.solution_service());
    let deleted_id = uc
        .run(user.uid, req.solution_id)
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
    let solutions = uc
        .run(problem_id.to_string(), sort)
        .await
        .map_err(|e| e.to_http_error())?;
    let ret: Vec<_> = solutions
        .into_iter()
        .map(GetSolutionsByProblemIdResponse::from)
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
    ApiQuery(req): ApiQuery<GetSolutionBySolutionIdRequest>,
) -> Result<Json<ApiResponse<GetSolutionBySolutionIdResponse>>, HttpError> {
    let uc = GetSolutionBySolutionIdUsecase::new(registry.solution_service());
    let solution = uc
        .run(req.solution_id)
        .await
        .map_err(|e| e.to_http_error())?;
    Ok(Json(ApiResponse::ok(solution.into())))
}

pub async fn vote_solution_handler(
    State(registry): State<Registry>,
    AuthUser(user): AuthUser,
    ApiJson(req): ApiJson<VoteSolutionRequest>,
) -> Result<Json<ApiResponse<VoteSolutionResponse>>, HttpError> {
    let uc = VoteSolutionUsecase::new(registry.solution_tx_manager(), registry.solution_service());
    uc.run(user.uid, req.solution_id)
        .await
        .map_err(|e| e.to_http_error())?;
    Ok(Json(ApiResponse::ok(VoteSolutionResponse::liked(
        req.solution_id,
    ))))
}

pub async fn unvote_solution_handler(
    State(registry): State<Registry>,
    AuthUser(user): AuthUser,
    ApiQuery(req): ApiQuery<UnvoteSolutionRequest>,
) -> Result<Json<ApiResponse<UnvoteSolutionResponse>>, HttpError> {
    let uc =
        UnvoteSolutionUsecase::new(registry.solution_tx_manager(), registry.solution_service());
    uc.run(user.uid, req.solution_id)
        .await
        .map_err(|e| e.to_http_error())?;
    Ok(Json(ApiResponse::ok(UnvoteSolutionResponse::unliked(
        req.solution_id,
    ))))
}

pub async fn get_solution_votes_count_handler(
    State(registry): State<Registry>,
    ApiQuery(req): ApiQuery<GetSolutionVotesCountRequest>,
) -> Result<Json<ApiResponse<GetSolutionVotesCountResponse>>, HttpError> {
    let uc = GetSolutionVotesCountUsecase::new(registry.solution_service());
    let count = uc
        .run(req.solution_id)
        .await
        .map_err(|e| e.to_http_error())?;
    Ok(Json(ApiResponse::ok(GetSolutionVotesCountResponse::new(
        req.solution_id,
        count,
    ))))
}

pub async fn get_my_vote_status_handler(
    State(registry): State<Registry>,
    AuthUser(user): AuthUser,
    ApiQuery(req): ApiQuery<GetMyVoteStatusRequest>,
) -> Result<Json<ApiResponse<GetMyVoteStatusResponse>>, HttpError> {
    let uc = GetMyVoteStatusUsecase::new(registry.solution_service());
    let liked = uc
        .run(user.uid, req.solution_id)
        .await
        .map_err(|e| e.to_http_error())?;
    Ok(Json(ApiResponse::ok(GetMyVoteStatusResponse::new(
        req.solution_id,
        liked,
    ))))
}

pub async fn create_comment_handler(
    State(registry): State<Registry>,
    AuthUser(user): AuthUser,
    ApiJson(req): ApiJson<CreateCommentRequest>,
) -> Result<Json<ApiResponse<CreateCommentResponse>>, HttpError> {
    let uc = CreateCommentUsecase::new(registry.solution_tx_manager(), registry.solution_service());
    let input = CreateCommentInput {
        user_id: user.uid,
        solution_id: req.solution_id,
        body_md: req.body_md,
    };
    let created = uc.run(input).await.map_err(|e| e.to_http_error())?;
    Ok(Json(ApiResponse::ok(CreateCommentResponse::from(created))))
}

pub async fn update_comment_handler(
    State(registry): State<Registry>,
    AuthUser(user): AuthUser,
    ApiJson(req): ApiJson<UpdateCommentRequest>,
) -> Result<Json<ApiResponse<UpdateCommentResponse>>, HttpError> {
    let uc = UpdateCommentUsecase::new(registry.solution_tx_manager(), registry.solution_service());
    let input = from_req_for_update_comment(user.uid, req);
    let updated = uc.run(input).await.map_err(|e| e.to_http_error())?;
    Ok(Json(ApiResponse::ok(UpdateCommentResponse::from(updated))))
}

pub async fn delete_comment_handler(
    State(registry): State<Registry>,
    AuthUser(user): AuthUser,
    ApiQuery(req): ApiQuery<DeleteCommentRequest>,
) -> Result<Json<ApiResponse<DeleteCommentResponse>>, HttpError> {
    let uc = DeleteCommentUsecase::new(registry.solution_tx_manager(), registry.solution_service());
    let deleted_id = uc
        .run(user.uid, req.comment_id)
        .await
        .map_err(|e| e.to_http_error())?;
    Ok(Json(ApiResponse::ok(deleted_id.into())))
}

pub async fn get_comments_by_solution_id_handler(
    State(registry): State<Registry>,
    ApiQuery(req): ApiQuery<GetCommentsBySolutionIdRequest>,
) -> Result<Json<ApiResponse<Vec<GetCommentsBySolutionIdResponse>>>, HttpError> {
    let uc = GetCommentsBySolutionIdUsecase::new(registry.solution_service());
    let comments = uc
        .run(req.solution_id)
        .await
        .map_err(|e| e.to_http_error())?;
    let ret: Vec<_> = comments
        .into_iter()
        .map(GetCommentsBySolutionIdResponse::from)
        .collect();
    Ok(Json(ApiResponse::ok(ret)))
}
