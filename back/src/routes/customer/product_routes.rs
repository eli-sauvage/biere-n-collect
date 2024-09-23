use axum::{routing::get, Json, Router};
use serde::Serialize;

use crate::{
    admin::bar_management::Bar,
    app::{product_variations::Variation, products},
    errors::ServerError,
    routes::AppState,
};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/get_bar_status", get(get_bar_status))
        .route("/get_available_products", get(get_available_products))
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

async fn get_available_products() -> Result<Json<Vec<products::Product>>, ServerError> {
    let products: Vec<products::Product> = products::get_all()
        .await?
        .into_iter()
        .map(|mut p| {
            p.variations = p
                .variations
                .into_iter()
                .filter(|v| v.available_to_order)
                .collect::<Vec<Variation>>();
            p
        })
        .filter(|p| p.variations.iter().any(|v| v.available_to_order))
        .collect();
    Ok(Json(products))
}
