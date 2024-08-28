use axum::{routing::get, Json, Router};

use crate::{
    admin::user::User,
    app::orders::{self, Order},
    errors::OrderManagementError,
    routes::AppState,
};

pub fn get_router() -> Router<AppState> {
    Router::new().route("/get_all", get(get_all_orders))
}

async fn get_all_orders(_user: User) -> Result<Json<Vec<Order>>, OrderManagementError> {
    let orders = orders::get_all_orders().await?;
    Ok(Json(orders))
}
