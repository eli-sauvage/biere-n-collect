use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    app::{
        orders::{Cart, Order, OrderId},
        stock,
        stripe::payment_intents::PaymentIntentStatus,
    },
    errors::{OrderProcessError, PaymentIntentError, ServerError},
    routes::CustomQuery as Query
};

use super::AppState;

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/get_available_stock", get(get_available_stock))
        .route("/validate_cart", post(validate_cart))
        .route("/get_payment_infos", get(get_payment_infos))
        .route("/get_payment_status", get(get_payment_status))
}

async fn get_available_stock() -> Result<Json<Vec<stock::Stock>>, ServerError> {
    let stock = stock::get_all_stocks().await?;
    Ok(Json(stock))
}

#[derive(Serialize)]
struct ValidateCartResponse {
    order_id: OrderId,
}
async fn validate_cart(
    Json(cart): Json<Cart>,
) -> Result<Json<ValidateCartResponse>, OrderProcessError> {
    let order_id = Order::generate_from_cart(cart).await?;
    Ok(Json(ValidateCartResponse { order_id }))
}

#[derive(Deserialize)]
struct OrderIdParam {
    order_id: OrderId,
}
#[derive(Serialize)]
struct PaymentInfos {
    client_secret: String,
    total_price: i32,
}
async fn get_payment_infos(
    params: Query<OrderIdParam>,
) -> Result<Json<PaymentInfos>, PaymentIntentError> {
    let mut order = Order::get(params.order_id)
        .await?
        .ok_or_else(|| PaymentIntentError::OrderNotFound(params.order_id))?;

    let intent = order.get_payment_intent().await?;

    Ok(Json(PaymentInfos {
        client_secret: intent.client_secret,
        total_price: intent.amount,
    }))
}

#[derive(Serialize)]
struct PaymentStatusResponse {
    status: PaymentIntentStatus,
}
async fn get_payment_status(
    params: Query<OrderIdParam>,
) -> Result<Json<PaymentStatusResponse>, PaymentIntentError> {
    let mut order = Order::get(params.order_id)
        .await?
        .ok_or_else(|| PaymentIntentError::OrderNotFound(params.order_id))?;

    let intent = order.get_payment_intent().await?;

    Ok(Json(PaymentStatusResponse {
        status: intent.status,
    }))
}
