use axum::{Json, extract::State};
use registry::Registry;
use shared::{error::http::HttpError, response::ApiResponse};
use usecase::solution::create::CreateSolutionUsecase;

use crate::{
    http::AuthUser, model::solution::create_solution::{from_req_for_input, CreateSolutionRequest, CreateSolutionResponse},
};

pub async fn create_solution_handler(
    State(registry): State<Registry>,
    AuthUser(user): AuthUser,
    Json(req): Json<CreateSolutionRequest>,
) -> Result<Json<ApiResponse<CreateSolutionResponse>>, HttpError> {
    let user_id = user.uid;
    let repo = CreateSolutionUsecase::new(registry.id_provider_port(), registry.solution_tx_manager());
    let input = from_req_for_input(user_id, req);
    let res = repo
        .run(input)
        .await
        .map_err(HttpError::from)?;

    Ok(Json(ApiResponse::ok(res.into())))
}

