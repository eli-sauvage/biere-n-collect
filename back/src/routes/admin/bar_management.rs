use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use tower_http::services::ServeDir;

use crate::{
    admin::bar_management::Bar,
    admin::user::AdminUser,
    errors::ServerError,
    routes::{extractors::CustomQuery as Query, reponders::OkEmptyResponse, AppState},
};

const REPORTS_DIR_PATH: &str = "./reports";
pub fn get_router() -> Router<AppState> {
    std::fs::create_dir_all(REPORTS_DIR_PATH).expect("could not create the reports directory");

    Router::new()
        .route("/", get(get_bar))
        .route("/open", post(open_bar))
        .route("/close", post(close_bar))
        .route("/set_closing_message", post(set_closing_message))
        .route("/list_reports", get(list_reports))
        .nest_service(
            "/reports",
            ServeDir::new("reports").append_index_html_on_directories(false),
        )
}

async fn get_bar(
    State(state): State<AppState>,
    _user: AdminUser,
) -> Result<Json<Bar>, ServerError> {
    let bar = Bar::get(&state.pool).await?;
    Ok(Json(bar))
}

async fn open_bar(
    State(state): State<AppState>,
    _user: AdminUser,
) -> Result<OkEmptyResponse, ServerError> {
    let mut bar = Bar::get(&state.pool).await?;
    bar.open(&state.pool).await?;
    Ok(OkEmptyResponse::new())
}
async fn close_bar(
    State(state): State<AppState>,
    _user: AdminUser,
) -> Result<OkEmptyResponse, ServerError> {
    let mut bar = Bar::get(&state.pool).await?;
    bar.close(&state.pool).await?;
    Ok(OkEmptyResponse::new())
}

#[derive(Deserialize)]
struct SetClosingMessageParams {
    closing_message: String,
}
async fn set_closing_message(
    State(state): State<AppState>,
    _user: AdminUser,
    params: Query<SetClosingMessageParams>,
) -> Result<OkEmptyResponse, ServerError> {
    let mut bar = Bar::get(&state.pool).await?;
    bar.set_closing_message(&state.pool, params.closing_message.clone())
        .await?;
    Ok(OkEmptyResponse::new())
}

async fn list_reports(_user: AdminUser) -> Result<Json<Vec<String>>, ServerError> {
    let dir = std::fs::read_dir(REPORTS_DIR_PATH).unwrap();
    let names: Vec<String> = dir
        .map(|f| f.unwrap().file_name().to_string_lossy().to_string())
        .collect();

    Ok(Json(names))
}
