use axum::{
    async_trait,
    body::Body,
    extract::{
        rejection::{JsonRejection, QueryRejection},
        FromRequest, FromRequestParts, Query, Request,
    },
    http::{request::Parts, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::de::DeserializeOwned;

use crate::errors::ErrorResponse;
use std::ops::Deref;

pub struct CustomQuery<T>(pub Query<T>);
#[async_trait]
impl<T, S> FromRequestParts<S> for CustomQuery<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = CustomQueryRejection;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let q = Query::from_request_parts(parts, state)
            .await
            .map_err(CustomQueryRejection)?;
        Ok(CustomQuery(q))
    }
}

impl<T> Deref for CustomQuery<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct CustomQueryRejection(pub QueryRejection);
impl IntoResponse for CustomQueryRejection {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::BAD_REQUEST,
            ErrorResponse::json(self.0.body_text()),
        )
            .into_response()
    }
}

pub struct CustomJsonExtractor<T>(pub Json<T>);
#[async_trait]
impl<T, S> FromRequest<S> for CustomJsonExtractor<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = CustomJsonRejection;
    async fn from_request(request: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let json = Json::from_request(request, state)
            .await
            .map_err(CustomJsonRejection)?;

        Ok(CustomJsonExtractor(json))
    }
}
impl<T> Deref for CustomJsonExtractor<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct CustomJsonRejection(pub JsonRejection);
impl IntoResponse for CustomJsonRejection {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::BAD_REQUEST,
            ErrorResponse::json(self.0.body_text()),
        )
            .into_response()
    }
}
