use axum::{Json, extract::State};
use registry::Registry;
use shared::{error::http::HttpError, response::ApiResponse};
use usecase::user::{
    create_user::CreateUserUsecase, delete_me::DeleteMeUsecase, get_me::GetMeUsecase,
    revoke_tokens::RevokeTokensUsecase,
};

use crate::{
    error::ToHttpError,
    http::AuthUser,
    model::user::create_user::{
        CreateUserRequest, CreateUserResponse, try_from_create_user_request_for_create_user_input,
    },
    model::user::delete_me::DeleteMeResponse,
    model::user::get_me::GetMeResponse,
    model::user::revoke_tokens::RevokeTokensResponse,
};

pub async fn create_user_handler(
    State(registry): State<Registry>,
    AuthUser(user): AuthUser,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<ApiResponse<CreateUserResponse>>, HttpError> {
    let uid = user.uid;
    let create_user_input = try_from_create_user_request_for_create_user_input(req, uid)
        .map_err(|e| e.to_http_error())?;
    let repo = CreateUserUsecase::new(registry.user_repository());
    let res = repo
        .run(create_user_input)
        .await
        .map_err(|e| e.to_http_error())?;

    Ok(Json(ApiResponse::ok(res.into())))
}

pub async fn get_me_handler(
    State(registry): State<Registry>,
    AuthUser(user): AuthUser,
) -> Result<Json<ApiResponse<GetMeResponse>>, HttpError> {
    let uc = GetMeUsecase::new(registry.user_repository());
    let me = uc.run(user.uid).await.map_err(|e| e.to_http_error())?;
    Ok(Json(ApiResponse::ok(me.into())))
}

pub async fn delete_me_handler(
    State(registry): State<Registry>,
    AuthUser(user): AuthUser,
) -> Result<Json<ApiResponse<DeleteMeResponse>>, HttpError> {
    let uc = DeleteMeUsecase::new(registry.user_repository());
    let deleted = uc.run(user.uid).await.map_err(|e| e.to_http_error())?;
    Ok(Json(ApiResponse::ok(deleted.into())))
}

pub async fn revoke_tokens_handler(
    State(registry): State<Registry>,
    AuthUser(user): AuthUser,
) -> Result<Json<ApiResponse<RevokeTokensResponse>>, HttpError> {
    let uc = RevokeTokensUsecase::new(registry.user_repository());
    let revoked = uc.run(user.uid).await.map_err(|e| e.to_http_error())?;
    Ok(Json(ApiResponse::ok(revoked.into())))
}
