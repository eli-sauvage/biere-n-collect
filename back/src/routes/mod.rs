pub(crate) mod admin;
pub(crate) mod order_routes;

use axum::{
    async_trait,
    body::Body,
    extract::{
        rejection::{JsonRejection, QueryRejection},
        FromRequest, FromRequestParts, Query, Request,
    },
    http::{request::Parts, HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::CookieJar;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

use crate::{
    admin::challenge::ChallengeManager,
    app::config::config,
    errors::{ErrorResponse, ServerError},
};
use std::{env, ops::Deref, sync::Arc};

pub async fn get_config() -> Result<Json<Value>, ServerError> {
    let conf = config().read().await;

    Ok(Json(
        json!({"publishable_key": conf.stripe_publishable_key()}),
    ))
}

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

pub struct InnerState {
    pub challenge_manager: ChallengeManager,
}
pub type AppState = Arc<InnerState>;

pub fn generate_app_state(challenge_manager: ChallengeManager) -> AppState {
    Arc::new(InnerState { challenge_manager })
}

pub async fn cors(request: Request, next: Next) -> Response {
    let mut response = if request.method() == axum::http::Method::OPTIONS {
        (StatusCode::OK, "").into_response()
    } else {
        next.run(request).await
    };
    let headers = response.headers_mut();

    headers.insert(
        "Access-Control-Allow-Origin",
        HeaderValue::from_str(&env::var("VITE_SITE_URL").unwrap()).unwrap(),
    );
    headers.insert(
        "Access-Control-Allow-Headers",
        HeaderValue::from_static("Content-Type,Authorization"),
    );
    headers.insert(
        "Access-Control-Allow-Methods",
        HeaderValue::from_static("GET,PUT,POST,DELETE,OPTIONS,PATCH"),
    );
    headers.insert(
        "Access-Control-Allow-Credentials",
        HeaderValue::from_static("true"),
    );
    response
}

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
