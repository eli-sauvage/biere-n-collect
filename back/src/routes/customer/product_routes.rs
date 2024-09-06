use axum::{routing::get, Json, Router};
use serde::Serialize;

use crate::{
    admin::bar_management::Bar,
    app::{
        product_categories::{self, Category},
        products,
    },
    errors::ServerError,
    routes::AppState,
};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/get_bar_status", get(get_bar_status))
        .route("/get_available_stock", get(get_available_stock))
        .route("/get_categories", get(get_categories))
}

#[derive(Serialize)]
struct BarStatusResponse {
    is_open: bool,
    closed_message: Option<String>,
}
async fn get_bar_status() -> Result<Json<BarStatusResponse>, ServerError> {
    let bar = Bar::get().await?;
    let res = BarStatusResponse {
        is_open: bar.is_open,
        closed_message: if bar.is_open {
            None
        } else {
            Some(bar.closing_message)
        },
    };

    Ok(Json(res))
}

async fn get_available_stock() -> Result<Json<Vec<products::Product>>, ServerError> {
    let products = products::get_all().await?;
    Ok(Json(products))
}

async fn get_categories() -> Result<Json<Vec<Category>>, ServerError> {
    let categories = product_categories::get_all().await?;
    Ok(Json(categories))
}
