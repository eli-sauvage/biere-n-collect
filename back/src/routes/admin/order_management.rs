use core::fmt;
use std::str::FromStr;

use crate::{
    app::orders::{Order, OrderId},
    errors::ServerError,
    routes::{CustomQuery as Query, OkEmptyResponse},
    utils::serialize_time,
};
use axum::{
    routing::{get, patch},
    Json, Router,
};
use serde::{de, Deserialize, Deserializer, Serialize};
use sqlx::types::time::OffsetDateTime;

use crate::{
    admin::user::User,
    app::orders::{self, OrderDetailElement},
    errors::OrderManagementError,
    routes::AppState,
};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_by_id))
        .route("/by_receipt", get(get_by_receipt))
        .route("/search", get(search_orders))
        .route("/set_served", patch(set_served))
}

#[derive(Serialize)]
struct OrderResponse {
    id: OrderId,
    receipt: Option<String>,
    served: bool,
    #[serde(serialize_with = "serialize_time")]
    timestamp: OffsetDateTime,
    user_email: Option<String>,
    detail: Vec<OrderDetailElement>,
    total_price: i32,
}
impl OrderResponse {
    pub async fn from_order(order: Order) -> Result<Self, ServerError> {
        let details = order.get_details().await?;
        let total_price = order.get_full_price().await?;
        let res = OrderResponse {
            id: order.id,
            receipt: order.receipt.as_deref().cloned(),
            served: order.served,
            timestamp: order.timestamp,
            user_email: order.user_email,
            detail: details,
            total_price,
        };
        Ok(res)
    }
}

#[derive(Deserialize)]
struct GetOrderParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    email: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    date_begin: Option<i64>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    date_end: Option<i64>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    receipt: Option<String>,
}
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

async fn search_orders(
    _user: User,
    params: Query<GetOrderParams>,
) -> Result<Json<Vec<OrderResponse>>, OrderManagementError> {
    let date_begin = params
        .date_begin
        .map(|ts| OffsetDateTime::from_unix_timestamp(ts / 1000))
        .transpose()
        .map_err(|_| OrderManagementError::InvalidDate)?;
    let date_end = params
        .date_end
        .map(|ts| OffsetDateTime::from_unix_timestamp(ts / 1000))
        .transpose()
        .map_err(|_| OrderManagementError::InvalidDate)?;
    println!("begin = {date_begin:?}");
    let orders = orders::search_orders(
        params.email.as_deref(),
        date_begin,
        date_end,
        params.receipt.as_deref(),
    )
    .await?;
    let mut res: Vec<OrderResponse> = vec![];
    for order in orders {
        res.push(OrderResponse::from_order(order).await?);
    }
    Ok(Json(res))
}

#[derive(Deserialize)]
struct GetByReceiptParams {
    receipt: String,
}
async fn get_by_receipt(
    _user: User,
    params: Query<GetByReceiptParams>,
) -> Result<Json<OrderResponse>, OrderManagementError> {
    let order = Order::get_by_receipt(&params.receipt)
        .await?
        .ok_or_else(|| OrderManagementError::OrderNotFound)?;
    let res = OrderResponse::from_order(order).await?;

    Ok(Json(res))
}

#[derive(Deserialize)]
struct GetByIdParams {
    id: OrderId,
}
async fn get_by_id(
    _user: User,
    params: Query<GetByIdParams>,
) -> Result<Json<OrderResponse>, OrderManagementError> {
    let order = Order::get(params.id)
        .await?
        .ok_or_else(|| OrderManagementError::OrderNotFound)?;
    let res = OrderResponse::from_order(order).await?;

    Ok(Json(res))
}

#[derive(Deserialize)]
struct MarkAsPaidParams {
    order_id: OrderId,
    new_served: bool,
}
async fn set_served(
    _user: User,
    params: Query<MarkAsPaidParams>,
) -> Result<OkEmptyResponse, OrderManagementError> {
    let mut order = Order::get(params.order_id)
        .await?
        .ok_or_else(|| OrderManagementError::OrderNotFound)?;
    order.set_served(params.new_served).await?;

    Ok(OkEmptyResponse::new())
}
