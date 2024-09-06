use axum::Router;

use super::AppState;

mod auth;
mod bar_management;
mod order_management;
mod stock;
mod user_management;

pub fn get_router() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::get_router())
        .nest("/users", user_management::get_router())
        .nest("/stock", stock::get_router())
        .nest("/orders", order_management::get_router())
        .nest("/bar", bar_management::get_router())
}
