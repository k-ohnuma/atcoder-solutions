use axum::{extract::State, http::StatusCode};
use registry::Registry;

pub async fn health_check(State(_registry): State<Registry>) -> StatusCode {
    StatusCode::OK
}

// pub async fn health_check_db() -> 
