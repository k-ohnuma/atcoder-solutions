use axum::{extract::State, http::StatusCode};
use registry::Registry;

pub async fn health_check(State(_registry): State<Registry>) -> StatusCode {
    StatusCode::OK
}

pub async fn health_check_db(State(registry): State<Registry>) -> StatusCode {
    if registry.health_check_repository().check_db().await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
} 
