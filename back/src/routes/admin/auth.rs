use axum::{
    extract::{FromRequestParts, Query, Request, State},
    routing::{delete, get, post},
    Json, Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use serde::{Deserialize, Serialize};

use crate::{
    admin::{
        auth::Session,
        user::{Role, User},
    },
    errors::SessionError,
    routes::AppState,
};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/get_current", get(get_auth))
        .route("/delete_current", delete(delete_current))
        .route("/challenge/create", post(create_challenge))
        .route("/challenge/verify", post(verify_challenge))
}

#[derive(Serialize, Default)]
struct Auth {
    authenticated: bool,
    role: Option<Role>,
    email: Option<String>,
    error: Option<String>,
}
async fn get_auth(request: Request) -> Json<Auth> {
    let mut parts = request.into_parts().0;
    match User::from_request_parts(&mut parts, &()).await {
        Ok(user) => Json(Auth {
            authenticated: true,
            role: Some(user.role),
            email: Some(user.email),
            ..Default::default()
        }),
        Err(e) => Json(Auth {
            authenticated: false,
            error: Some(e.to_string()),
            ..Default::default()
        }),
    }
}

async fn delete_current(_user: User, cookie_jar: CookieJar) -> Result<CookieJar, SessionError> {
    let session = cookie_jar
        .get("session")
        .ok_or_else(|| SessionError::SessionNotFound)?
        .to_string();

    let cookie_jar = cookie_jar.remove("session");

    Session::delete_if_exists(&session).await?;

    Ok(cookie_jar)
}

#[derive(Deserialize)]
struct CreateChallengeParams {
    email: String,
}
async fn create_challenge(
    State(state): State<AppState>,
    params: Query<CreateChallengeParams>,
) -> Result<(), SessionError> {
    state
        .challenge_manager
        .create_challenge(&params.email)
        .await?;
    Ok(())
}

#[derive(Deserialize)]
struct VerifyChallengeParams {
    email: String,
    code: String,
}
async fn verify_challenge(
    cookies: CookieJar,
    State(state): State<AppState>,
    params: Query<VerifyChallengeParams>,
) -> Result<CookieJar, SessionError> {
    let session = state
        .challenge_manager
        .verify_challenge(&params.email, &params.code)
        .await?;

    let cookie = Cookie::build(("session", session.uuid))
        .expires(session.expires)
        .path("/")
        .secure(true);

    let cookies = cookies.add(cookie);
    Ok(cookies)
}