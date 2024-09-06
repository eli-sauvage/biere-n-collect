use crate::routes::AppState;
use axum::Router;

mod category_management;
mod product_management;
mod product_variations_management;

pub fn get_router() -> Router<AppState> {
    Router::new()
        .nest("/products", product_management::get_router())
        .nest("/categories", category_management::get_router())
        .nest("/variations", product_variations_management::get_router())
}
