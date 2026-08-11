use axum::{
    extract::{FromRequest, FromRequestParts, Json, Query, Request},
    http::request::Parts,
};
use serde::de::DeserializeOwned;

use crate::error::HttpError;

pub struct ApiJson<T>(pub T);
pub struct ApiQuery<T>(pub T);

impl<S, T> FromRequest<S> for ApiJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Send,
{
    type Rejection = HttpError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        Json::<T>::from_request(req, state)
            .await
            .map(|Json(value)| ApiJson(value))
            .map_err(|e| HttpError::BadRequest(e.body_text()))
    }
}

impl<S, T> FromRequestParts<S> for ApiQuery<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Send,
{
    type Rejection = HttpError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Query::<T>::from_request_parts(parts, state)
            .await
            .map(|Query(value)| ApiQuery(value))
            .map_err(|e| HttpError::BadRequest(e.to_string()))
    }
}
