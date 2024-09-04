mod admin;
mod app;
mod db;

use admin::challenge::ChallengeManager;
pub(crate) use db::db;
mod errors;
mod routes;

use axum::{middleware, routing::get, Router};
use errors::ServerError;
use routes::{generate_app_state, handler_404};
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    dotenvy::dotenv().expect("could not load env from .env file");
    db::setup_db_and_migrate().await;
    let challenge_manager = ChallengeManager::new();
    let state = generate_app_state(challenge_manager);

    let app = Router::new()
        .route("/api/config", get(routes::get_config))
        .nest("/api/order", routes::order_routes::get_router())
        .nest("/api/admin", routes::admin::get_router())
        .nest_service("/", ServeDir::new("dist"))
        .nest_service("/login", ServeFile::new("dist/index.html"))
        .nest_service("/checkout", ServeFile::new("dist/index.html"))
        .nest_service("/return", ServeFile::new("dist/index.html"))
        .nest_service("/serveur", ServeFile::new("dist/index.html"))
        .nest_service("/admin", ServeFile::new("dist/index.html"))
        .fallback(handler_404)
        .with_state(state)
        .layer(middleware::from_fn(routes::cors));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
