use axum::Router;

use super::AppState;

pub(crate) mod order_routes;
pub(crate) mod product_routes;

pub fn get_router() -> Router<AppState> {
    Router::new()
        .nest("/", order_routes::get_router())
        .nest("/", product_routes::get_router())
}
