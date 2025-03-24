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
    routes::{reponders::OkEmptyResponse, AppState},
};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/get_current", get(get_auth))
        .route("/delete_current", delete(delete_current))
        .route("/challenge/create", post(create_challenge))
        .route("/challenge/verify", get(verify_challenge))
}

#[derive(Serialize, Default)]
struct Auth {
    authenticated: bool,
    role: Option<Role>,
    email: Option<String>,
    error: Option<String>,
}
async fn get_auth(State(state): State<AppState>, request: Request) -> Json<Auth> {
    let (mut parts, _) = request.into_parts();

    match User::from_request_parts(&mut parts, &state).await {
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

async fn delete_current(
    _user: User,
    State(state): State<AppState>,
    cookie_jar: CookieJar,
) -> Result<OkEmptyResponse, SessionError> {
    println!("delete");
    let session = cookie_jar
        .get("session")
        .ok_or_else(|| SessionError::SessionNotFound)?
        .to_string();

    let cookie_jar = cookie_jar.remove(Cookie::build("session").path("/"));

    Session::delete_if_exists(&state.pool, &session).await?;

    Ok(OkEmptyResponse::new_with_cookies(cookie_jar))
}

#[derive(Deserialize)]
struct CreateChallengeParams {
    email: String,
}
async fn create_challenge(
    State(state): State<AppState>,
    params: Query<CreateChallengeParams>,
) -> Result<OkEmptyResponse, SessionError> {
    let message = state
        .challenge_manager
        .create_challenge(&state.pool, &params.email)
        .await?;

    match state.mail_manager.send_mail(message).await {
        Ok(()) => {}
        Err(e) if cfg!(not(debug_assertions)) => return Err(e.into()),
        Err(_) => {
            println!("could not send auth email, discarding error because we are in debug mode");
        }
    }
    Ok(OkEmptyResponse::new())
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
) -> Result<OkEmptyResponse, SessionError> {
    let challenge_succedeed = state
        .challenge_manager
        .verify_challenge(&params.email, &params.code)
        .await?;

    if challenge_succedeed {
        let session = Session::new(&state.pool, params.email.clone()).await?;
        let cookie = Cookie::build(("session", session.uuid))
            .expires(session.expires)
            .path("/")
            .secure(true);

        let cookies = cookies.add(cookie);
        Ok(OkEmptyResponse::new_with_cookies(cookies))
    } else {
        Err(SessionError::ChallengeFailed(params.email.clone()))
    }
}
