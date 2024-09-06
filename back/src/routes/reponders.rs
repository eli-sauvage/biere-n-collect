use axum::{
    extract::Request,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::CookieJar;
use serde_json::json;

use crate::errors::ErrorResponse;

pub async fn handler_404(request: Request) -> Response {
    let path = request.uri().path();
    (
        StatusCode::NOT_FOUND,
        ErrorResponse::json(format!("404: url {} not found", path)),
    )
        .into_response()
}

pub struct OkEmptyResponse {
    pub cookies: Option<CookieJar>,
}
impl OkEmptyResponse {
    pub fn new() -> Self {
        OkEmptyResponse { cookies: None }
    }
    pub fn new_with_cookies(cookies: CookieJar) -> Self {
        OkEmptyResponse {
            cookies: Some(cookies),
        }
    }
}
impl IntoResponse for OkEmptyResponse {
    fn into_response(self) -> Response {
        if let Some(cookies) = self.cookies {
            (StatusCode::OK, cookies, Json(json!({}))).into_response()
        } else {
            (StatusCode::OK, Json(json!({}))).into_response()
        }
    }
}
