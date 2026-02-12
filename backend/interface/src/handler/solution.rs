use axum::{
    Json,
    extract::{Query, State},
};
use registry::Registry;
use shared::{error::http::HttpError, response::ApiResponse};
use usecase::solution::{
    create::CreateSolutionUsecase, get_by_problem_id::GetSolutionsByProblemIdUsecase,
    get_by_solution_id::GetSolutionBySolutionIdUsecase, get_my_vote_status::GetMyVoteStatusUsecase,
    get_votes_count::GetSolutionVotesCountUsecase, unvote::UnvoteSolutionUsecase,
    vote::VoteSolutionUsecase,
};

use crate::{
    error::ToHttpError,
    http::AuthUser,
    model::solution::{
        create_solution::{CreateSolutionRequest, CreateSolutionResponse, from_req_for_input},
        get_my_vote_status::{GetMyVoteStatusRequest, GetMyVoteStatusResponse},
        get_solution_by_solution_id::{
            GetSolutionBySolutionIdRequest, GetSolutionBySolutionIdResponse,
        },
        get_solution_votes_count::{GetSolutionVotesCountRequest, GetSolutionVotesCountResponse},
        get_solutions_by_problem_id::{
            GetSolutionsByProblemIdRequest, GetSolutionsByProblemIdResponse,
        },
        unvote_solution::{UnvoteSolutionRequest, UnvoteSolutionResponse},
        vote_solution::{VoteSolutionRequest, VoteSolutionResponse},
    },
};

pub async fn create_solution_handler(
    State(registry): State<Registry>,
    AuthUser(user): AuthUser,
    Json(req): Json<CreateSolutionRequest>,
) -> Result<Json<ApiResponse<CreateSolutionResponse>>, HttpError> {
    let user_id = user.uid;
    let repo =
        CreateSolutionUsecase::new(registry.id_provider_port(), registry.solution_tx_manager());
    let input = from_req_for_input(user_id, req);
    let res = repo.run(input).await.map_err(|e| e.to_http_error())?;

    Ok(Json(ApiResponse::ok(res.into())))
}
pub async fn get_solutions_by_problems_id_handler(
    State(registry): State<Registry>,
    Query(req): Query<GetSolutionsByProblemIdRequest>,
) -> Result<Json<ApiResponse<Vec<GetSolutionsByProblemIdResponse>>>, HttpError> {
    let uc = GetSolutionsByProblemIdUsecase::new(registry.solution_service());
    let solutions = uc
        .run(req.problem_id.to_string(), req.list_sort())
        .await
        .map_err(|e| e.to_http_error())?;
    let ret: Vec<_> = solutions
        .into_iter()
        .map(GetSolutionsByProblemIdResponse::from)
        .collect();
    Ok(Json(ApiResponse::ok(ret)))
}

pub async fn get_solution_by_solution_id_handler(
    State(registry): State<Registry>,
    Query(req): Query<GetSolutionBySolutionIdRequest>,
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
    Json(req): Json<VoteSolutionRequest>,
) -> Result<Json<ApiResponse<VoteSolutionResponse>>, HttpError> {
    let uc = VoteSolutionUsecase::new(registry.solution_tx_manager());
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
    Query(req): Query<UnvoteSolutionRequest>,
) -> Result<Json<ApiResponse<UnvoteSolutionResponse>>, HttpError> {
    let uc = UnvoteSolutionUsecase::new(registry.solution_tx_manager());
    uc.run(user.uid, req.solution_id)
        .await
        .map_err(|e| e.to_http_error())?;
    Ok(Json(ApiResponse::ok(UnvoteSolutionResponse::unliked(
        req.solution_id,
    ))))
}

pub async fn get_solution_votes_count_handler(
    State(registry): State<Registry>,
    Query(req): Query<GetSolutionVotesCountRequest>,
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
    Query(req): Query<GetMyVoteStatusRequest>,
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
