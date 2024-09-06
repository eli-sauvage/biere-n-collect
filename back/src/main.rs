mod admin;
mod app;
mod utils;

use admin::challenge::ChallengeManager;
pub(crate) use utils::db;
mod errors;
mod routes;

use axum::{middleware, Router};
use errors::ServerError;
use routes::generate_app_state;
use tokio::signal;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    dotenvy::dotenv().expect("could not load env from .env file");
    utils::setup_db_and_migrate().await;
    let challenge_manager = ChallengeManager::new();
    let state = generate_app_state(challenge_manager);

    let app = Router::new()
        .nest("/api", routes::customer::get_router())
        .nest("/api/admin", routes::admin::get_router())
        .nest_service("/", ServeDir::new("dist"))
        .nest_service("/login", ServeFile::new("dist/index.html"))
        .nest_service("/checkout", ServeFile::new("dist/index.html"))
        .nest_service("/return", ServeFile::new("dist/index.html"))
        .nest_service("/serveur", ServeFile::new("dist/index.html"))
        .nest_service("/admin", ServeFile::new("dist/index.html"))
        .fallback(routes::reponders::handler_404)
        .with_state(state)
        .layer(middleware::from_fn(routes::cors::cors));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };
    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
    println!("Terminate signal received");
}
