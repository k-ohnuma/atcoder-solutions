use axum::{
    Json,
    extract::{Query, State},
};
use registry::Registry;
use shared::{error::http::HttpError, response::ApiResponse};
use usecase::solution::{
    create::CreateSolutionUsecase, get_by_problem_id::GetSolutionsByProblemIdUsecase,
    get_by_solution_id::GetSolutionBySolutionIdUsecase,
};

use crate::{
    http::AuthUser,
    model::solution::{
        create_solution::{CreateSolutionRequest, CreateSolutionResponse, from_req_for_input},
        get_solution_by_solution_id::{
            GetSolutionBySolutionIdRequest, GetSolutionBySolutionIdResponse,
        },
        get_solutions_by_problem_id::{
            GetSolutionsByProblemIdRequest, GetSolutionsByProblemIdResponse,
        },
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
    let res = repo.run(input).await.map_err(HttpError::from)?;

    Ok(Json(ApiResponse::ok(res.into())))
}
pub async fn get_solutions_by_problems_id_handler(
    State(registry): State<Registry>,
    Query(req): Query<GetSolutionsByProblemIdRequest>,
) -> Result<Json<ApiResponse<Vec<GetSolutionsByProblemIdResponse>>>, HttpError> {
    let uc = GetSolutionsByProblemIdUsecase::new(registry.solution_service());
    let solutions = uc.run(req.problem_id).await?;
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
    let solution = uc.run(req.solution_id).await?;
    Ok(Json(ApiResponse::ok(solution.into())))
}
