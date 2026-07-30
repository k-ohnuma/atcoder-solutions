use axum::extract::{Path, State};
use registry::Registry;
use usecase::solution::{
    get_my_vote_status::GetMyVoteStatusUsecase, get_votes_count::GetSolutionVotesCountUsecase,
    unvote::UnvoteSolutionUsecase, vote::VoteSolutionUsecase,
};
use uuid::Uuid;

use crate::{
    error::IntoApiResult,
    http::{ApiResult, AuthUser},
    model::solution::{
        get_my_vote_status::GetMyVoteStatusResponse,
        get_solution_votes_count::GetSolutionVotesCountResponse,
        unvote_solution::UnvoteSolutionResponse, vote_solution::VoteSolutionResponse,
    },
    response::ApiResponse,
};

pub async fn vote_solution_handler(
    State(registry): State<Registry>,
    Path(solution_id): Path<Uuid>,
    AuthUser(user): AuthUser,
) -> ApiResult<VoteSolutionResponse> {
    let uc = VoteSolutionUsecase::new(registry.solution_tx_manager(), registry.solution_service());
    uc.run(user.uid, solution_id).await.into_api_result()?;
    Ok(ApiResponse::ok(VoteSolutionResponse::liked(solution_id)))
}

pub async fn unvote_solution_handler(
    State(registry): State<Registry>,
    Path(solution_id): Path<Uuid>,
    AuthUser(user): AuthUser,
) -> ApiResult<UnvoteSolutionResponse> {
    let uc =
        UnvoteSolutionUsecase::new(registry.solution_tx_manager(), registry.solution_service());
    uc.run(user.uid, solution_id).await.into_api_result()?;
    Ok(ApiResponse::ok(UnvoteSolutionResponse::unliked(
        solution_id,
    )))
}

pub async fn get_solution_votes_count_handler(
    State(registry): State<Registry>,
    Path(solution_id): Path<Uuid>,
) -> ApiResult<GetSolutionVotesCountResponse> {
    let uc = GetSolutionVotesCountUsecase::new(registry.solution_service());
    let count = uc.run(solution_id).await.into_api_result()?;
    Ok(ApiResponse::ok(GetSolutionVotesCountResponse::new(
        solution_id,
        count,
    )))
}

pub async fn get_my_vote_status_handler(
    State(registry): State<Registry>,
    Path(solution_id): Path<Uuid>,
    AuthUser(user): AuthUser,
) -> ApiResult<GetMyVoteStatusResponse> {
    let uc = GetMyVoteStatusUsecase::new(registry.solution_service());
    let liked = uc.run(user.uid, solution_id).await.into_api_result()?;
    Ok(ApiResponse::ok(GetMyVoteStatusResponse::new(
        solution_id,
        liked,
    )))
}
