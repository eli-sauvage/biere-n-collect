use axum::{
    body::Body,
    http::HeaderValue,
    response::Response,
    routing::{get, patch, post},
    Json, Router,
};
use qrcode::render::svg;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};

use crate::{
    app::{
        orders::{Cart, Order, OrderDetailElement, OrderId},
        stock,
        stripe::payment_intents::PaymentIntentStatus,
    },
    errors::{OrderProcessError, PaymentIntentError, ServerError},
    routes::{CustomJsonExtractor as JsonExtractor, CustomQuery as Query},
};

use super::{AppState, OkEmptyResponse};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/get_available_stock", get(get_available_stock))
        .route("/validate_cart", post(validate_cart))
        .route("/get_payment_infos", get(get_payment_infos))
        .route("/set_email", patch(set_email))
        .route("/get_payment_status", get(get_payment_status))
        .route("/get_qr_code", get(get_qr_code))
}

async fn get_available_stock() -> Result<Json<Vec<stock::Stock>>, OrderProcessError> {
    let stock = stock::get_all_stocks().await?;
    Ok(Json(stock))
}

#[derive(Serialize)]
struct ValidateCartResponse {
    order_id: OrderId,
}
async fn validate_cart(
    JsonExtractor(Json(cart)): JsonExtractor<Cart>,
) -> Result<Json<ValidateCartResponse>, OrderProcessError> {
    let order_id = Order::generate_from_cart(cart).await?;
    Ok(Json(ValidateCartResponse { order_id }))
}

#[derive(Deserialize)]
struct PaymentInfosParams {
    order_id: OrderId,
}
#[derive(Serialize)]
struct PaymentInfos {
    client_secret: String,
    total_price: i32,
}
async fn get_payment_infos(
    params: Query<PaymentInfosParams>,
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

#[derive(Deserialize)]
struct SetEmailParams {
    client_secret: String,
    email: String,
}

async fn set_email(params: Query<SetEmailParams>) -> Result<OkEmptyResponse, PaymentIntentError> {
    let mut order = Order::get_from_client_secret(&params.client_secret)
        .await?
        .ok_or_else(|| PaymentIntentError::OrderNotFoundFromSecrets)?;
    order.set_email(&params.email).await?;
    println!("setting email");

    Ok(OkEmptyResponse::new())
}

#[derive(Deserialize)]
struct PaymentStatusParams {
    client_secret: String,
}
#[derive(Serialize)]
struct PaymentStatusResponse {
    status: PaymentIntentStatus,
    receipt: Option<String>,
    email: Option<String>,
    detail: Vec<OrderDetailElement>,
    total_price: i32,
}
async fn get_payment_status(
    params: Query<PaymentStatusParams>,
) -> Result<Json<PaymentStatusResponse>, PaymentIntentError> {
    let mut order = Order::get_from_client_secret(&params.client_secret)
        .await?
        .ok_or_else(|| PaymentIntentError::OrderNotFoundFromSecrets)?;

    let intent = order.get_payment_intent().await?;
    let total_price = intent.amount;

    let res = Json(PaymentStatusResponse {
        status: intent.status,
        receipt: order.receipt.as_deref().cloned(),
        email: order.user_email.clone(),
        detail: order.get_details().await?,
        total_price,
    });

    Ok(res)
}

async fn get_qr_code(params: Query<PaymentStatusParams>) -> Result<Response, PaymentIntentError> {
    let order = Order::get_from_client_secret(&params.client_secret)
        .await?
        .ok_or_else(|| PaymentIntentError::OrderNotFoundFromSecrets)?;
    let receipt = order.receipt.ok_or_else(|| PaymentIntentError::NoReceipt)?;

    let img = receipt
        .get_qr_code()?
        .render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#FFFFFF"))
        .build();

    let mut response = Response::new(Body::from(img));
    let headers = response.headers_mut();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("image/svg+xml"));
    Ok(response)
}
