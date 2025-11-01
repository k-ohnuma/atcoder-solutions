use axum::{Json, extract::State};
use registry::Registry;
use shared::{error::http::HttpError, response::ApiResponse};
use usecase::user::create_user::CreateUserUsecase;

use crate::{
    http::AuthUser,
    model::user::create_user::{
        CreateUserRequest, CreateUserResponse, try_from_create_user_request_for_create_user_input,
    },
};

pub async fn create_user_handler(
    State(registry): State<Registry>,
    AuthUser(user): AuthUser,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<ApiResponse<CreateUserResponse>>, HttpError> {
    let uid = user.uid;
    let create_user_input =
        try_from_create_user_request_for_create_user_input(req, uid).map_err(HttpError::from)?;
    let repo = CreateUserUsecase::new(registry.user_repository());
    let res = repo.run(create_user_input).await.map_err(HttpError::from)?;

    Ok(Json(ApiResponse::ok(res.into())))
}
