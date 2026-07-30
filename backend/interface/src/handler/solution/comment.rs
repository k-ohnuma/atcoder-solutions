use axum::extract::{Path, State};
use registry::Registry;
use usecase::{
    model::solution::create_comment::CreateCommentInput,
    solution::{
        create_comment::CreateCommentUsecase, delete_comment::DeleteCommentUsecase,
        get_comments_by_solution_id::GetCommentsBySolutionIdUsecase,
        update_comment::UpdateCommentUsecase,
    },
};
use uuid::Uuid;

use crate::{
    error::IntoApiResult,
    http::{ApiJson, ApiResult, AuthUser, TurnstileGuard},
    model::solution::{
        create_comment::{CreateCommentRequest, CreateCommentResponse},
        delete_comment::DeleteCommentResponse,
        get_comments_by_solution_id::GetCommentsBySolutionIdResponse,
        update_comment::{
            UpdateCommentRequest, UpdateCommentResponse,
            from_req_for_input as from_req_for_update_comment,
        },
    },
    response::ApiResponse,
};

pub async fn create_comment_handler(
    State(registry): State<Registry>,
    Path(solution_id): Path<Uuid>,
    AuthUser(user): AuthUser,
    _turnstile_guard: TurnstileGuard,
    ApiJson(req): ApiJson<CreateCommentRequest>,
) -> ApiResult<CreateCommentResponse> {
    let uc = CreateCommentUsecase::new(registry.solution_repository());
    let input = CreateCommentInput {
        user_id: user.uid,
        solution_id,
        body_md: req.body_md,
    };
    let created = uc.run(input).await.into_api_result()?;
    Ok(ApiResponse::ok(CreateCommentResponse::from(created)))
}

pub async fn update_comment_handler(
    State(registry): State<Registry>,
    Path(comment_id): Path<Uuid>,
    AuthUser(user): AuthUser,
    ApiJson(req): ApiJson<UpdateCommentRequest>,
) -> ApiResult<UpdateCommentResponse> {
    let uc = UpdateCommentUsecase::new(registry.solution_repository());
    let input = from_req_for_update_comment(user.uid, comment_id, req);
    let updated = uc.run(input).await.into_api_result()?;
    Ok(ApiResponse::ok(UpdateCommentResponse::from(updated)))
}

pub async fn delete_comment_handler(
    State(registry): State<Registry>,
    Path(comment_id): Path<Uuid>,
    AuthUser(user): AuthUser,
) -> ApiResult<DeleteCommentResponse> {
    let uc = DeleteCommentUsecase::new(registry.solution_repository());
    let deleted_id = uc.run(user.uid, comment_id).await.into_api_result()?;
    Ok(ApiResponse::ok(deleted_id.into()))
}

pub async fn get_comments_by_solution_id_handler(
    State(registry): State<Registry>,
    Path(solution_id): Path<Uuid>,
) -> ApiResult<Vec<GetCommentsBySolutionIdResponse>> {
    let uc = GetCommentsBySolutionIdUsecase::new(registry.solution_repository());
    let comments = uc.run(solution_id).await.into_api_result()?;
    let ret: Vec<_> = comments
        .into_iter()
        .map(GetCommentsBySolutionIdResponse::from)
        .collect();
    Ok(ApiResponse::ok(ret))
}
