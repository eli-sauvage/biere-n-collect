use axum::Router;

use super::AppState;

mod auth;
mod order_management;
mod stock_management;
mod user_management;

pub fn get_router() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::get_router())
        .nest("/users", user_management::get_router())
        .nest("/stock", stock_management::get_router())
        .nest("/orders", order_management::get_router())
}