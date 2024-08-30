use axum::{
    routing::{patch, post},
    Json, Router,
};
use serde::Deserialize;

use crate::{
    admin::user::AdminUser,
    app::stock::{self, Stock},
    errors::ManageStockError,
    routes::{AppState, CustomQuery as Query, OkEmptyResponse},
};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(insert_stock).put(update_stock).delete(delete_stock),
        )
        .route("/move", patch(move_stock))
}


#[derive(Deserialize)]
struct InsertStockParams {
    name: String,
    price: u32,
    quantity: u32,
    available: bool,
}
async fn insert_stock(
    _user: AdminUser,
    params: Query<InsertStockParams>,
) -> Result<OkEmptyResponse, ManageStockError> {
    stock::insert_stock(
        &params.name,
        params.price,
        params.quantity,
        params.available,
    )
    .await?;

    Ok(OkEmptyResponse::new())
}

async fn update_stock(
    _user: AdminUser,
    Json(new_stock): Json<Stock>,
) -> Result<OkEmptyResponse, ManageStockError> {
    if !stock::get_all_stocks()
        .await?
        .into_iter()
        .any(|stock| stock.product_id == new_stock.product_id)
    {
        return Err(ManageStockError::StockNotFound(new_stock.product_id));
    }

    stock::update_stock(new_stock).await?;

    Ok(OkEmptyResponse::new())
}

#[derive(Deserialize)]
struct DeleteStockParams {
    product_id: u32,
}
async fn delete_stock(
    _user: AdminUser,
    params: Query<DeleteStockParams>,
) -> Result<OkEmptyResponse, ManageStockError> {
    if !stock::get_all_stocks()
        .await?
        .into_iter()
        .any(|stock| stock.product_id == params.product_id)
    {
        return Err(ManageStockError::StockNotFound(params.product_id));
    }

    stock::delete_stock(params.product_id).await?;

    Ok(OkEmptyResponse::new())
}

#[derive(Deserialize)]
struct MoveStockParams {
    product_id: u32,
    direction: stock::MoveDirection,
}
async fn move_stock(
    _user: AdminUser,
    params: Query<MoveStockParams>,
) -> Result<OkEmptyResponse, ManageStockError> {
    if !stock::get_all_stocks()
        .await?
        .into_iter()
        .any(|stock| stock.product_id == params.product_id)
    {
        return Err(ManageStockError::StockNotFound(params.product_id));
    }

    stock::move_stock(params.direction, params.product_id).await?;

    Ok(OkEmptyResponse::new())
}
