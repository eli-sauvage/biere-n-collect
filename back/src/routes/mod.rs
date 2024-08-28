pub(crate) mod admin;
pub(crate) mod order_routes;

use axum::{
    async_trait,
    extract::{rejection::QueryRejection, FromRequestParts, Query},
    http::{request::Parts, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

use crate::{
    admin::challenge::ChallengeManager,
    errors::{ErrorResponse, ServerError},
};
use std::{env, ops::Deref, sync::Arc};

pub async fn get_config() -> Result<Json<Value>, ServerError> {
    let pub_key = env::var("STRIPE_PUBLISHABLE_KEY")
        .map_err(|e| ServerError::MissingEnv("STRIPE_PUBLISHABLE_KEY".into(), e))?;

    Ok(Json(json!({"publishable_key": pub_key})))
}

pub struct InnerState {
    pub challenge_manager: ChallengeManager,
}
pub type AppState = Arc<InnerState>;

pub fn generate_app_state(challenge_manager: ChallengeManager) -> AppState {
    Arc::new(InnerState { challenge_manager })
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
