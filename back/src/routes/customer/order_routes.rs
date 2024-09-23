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
use std::env;

use crate::{
    admin::bar_management::Bar,
    app::{
        orders::{Cart, Order, OrderDetailElement, OrderId},
        stripe::payment_intents::PaymentIntentStatus,
    },
    errors::{OrderProcessError, PaymentIntentError, ServerError},
    routes::{
        extractors::{CustomJsonExtractor as JsonExtractor, CustomQuery as Query},
        reponders::OkEmptyResponse,
        AppState,
    },
};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/get_stripe_pub_key", get(get_stripe_pub_key))
        .route("/validate_cart", post(validate_cart))
        .route("/get_payment_infos", get(get_payment_infos))
        .route("/set_email", patch(set_email))
        .route("/get_payment_status", get(get_payment_status))
        .route("/get_qr_code", get(get_qr_code))
}

#[derive(Serialize)]
struct StripePubKeyResponse {
    publishable_key: String,
}
async fn get_stripe_pub_key() -> Result<Json<StripePubKeyResponse>, ServerError> {
    let publishable_key = env::var("STRIPE_PUBLISHABLE_KEY")
        .map_err(|e| ServerError::MissingEnv("STRIPE_PUBLISHABLE_KEY".into(), e))?;

    Ok(Json(StripePubKeyResponse { publishable_key }))
}

#[derive(Serialize)]
struct ValidateCartResponse {
    order_id: OrderId,
}
async fn validate_cart(
    JsonExtractor(Json(cart)): JsonExtractor<Cart>,
) -> Result<Json<ValidateCartResponse>, OrderProcessError> {
    if !Bar::get().await?.is_open {
        return Err(OrderProcessError::BarIsClosed);
    }
    if cart.elements.iter().find(|e|e.quantity > 0).is_none(){
        return Err(OrderProcessError::EmptyOrder);
    }
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
    if !Bar::get().await?.is_open {
        return Err(PaymentIntentError::BarIsClosed);
    }
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
    if !Bar::get().await?.is_open {
        return Err(PaymentIntentError::BarIsClosed);
    }
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
