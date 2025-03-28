use std::{convert::Infallible, time::Duration};

use crate::{
    app::orders::{Order, OrderId},
    errors::ServerError,
    routes::{extractors::CustomQuery as Query, reponders::OkEmptyResponse},
    utils::{deserialize_empty_as_none, serialize_time},
};
use axum::{
    extract::State,
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
    routing::{get, patch},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{types::time::OffsetDateTime, SqlitePool};

use crate::{
    admin::user::User,
    app::orders::{self, OrderDetailElement},
    errors::OrderManagementError,
    routes::AppState,
};
use futures_util::{
    stream::{self, Stream},
    FutureExt, TryStreamExt,
};
use tokio_stream::{wrappers::BroadcastStream, StreamExt as _};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_by_id))
        .route("/by_receipt", get(get_by_receipt))
        .route("/search", get(search_orders))
        .route("/set_served", patch(set_served))
        .route("/notify_client", patch(notify_client))
        .route("/get_order_stream", get(sse_handler))
}

#[derive(Serialize)]
pub struct OrderResponse {
    id: OrderId,
    receipt: Option<String>,
    served: bool,
    client_notified: bool,
    #[serde(serialize_with = "serialize_time")]
    timestamp: OffsetDateTime,
    user_email: Option<String>,
    detail: Vec<OrderDetailElement>,
    total_price_ht: i32,
    total_price_ttc: i32,
}
impl OrderResponse {
    pub async fn from_order(pool: &SqlitePool, order: Order) -> Result<Self, ServerError> {
        let details = order.get_details(pool).await?;
        let total_price_ht = order.get_full_price_ht(pool).await?;
        let total_price_ttc = order.get_full_price_ttc(pool).await?;
        let res = OrderResponse {
            id: order.id,
            receipt: order.receipt.as_deref().cloned(),
            served: order.served,
            client_notified: order.client_notified,
            timestamp: order.timestamp,
            user_email: order.user_email,
            detail: details,
            total_price_ht,
            total_price_ttc,
        };
        Ok(res)
    }
}

#[derive(Deserialize)]
struct GetOrderParams {
    #[serde(default, deserialize_with = "deserialize_empty_as_none")]
    email: Option<String>,
    #[serde(default, deserialize_with = "deserialize_empty_as_none")]
    date_begin: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_empty_as_none")]
    date_end: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_empty_as_none")]
    receipt: Option<String>,
}

async fn search_orders(
    State(state): State<AppState>,
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
    let orders = orders::search_orders(
        &state.pool,
        params.email.as_deref(),
        date_begin,
        date_end,
        params.receipt.as_deref(),
    )
    .await?;
    let mut res: Vec<OrderResponse> = vec![];
    for order in orders {
        res.push(OrderResponse::from_order(&state.pool, order).await?);
    }
    Ok(Json(res))
}

async fn sse_handler(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, ServerError>>> {
    let rx = state.new_orders_channel.subscribe();
    let stream = BroadcastStream::new(rx);
    let async_state = state.clone();
    let stream = stream
        .hen(async move |or| {
            let state = async_state.clone();
            OrderResponse::from_order(&state.pool, or.unwrap()).await
        })
        .map(|e| Ok(Event::default().data("a")));
    /* let stream = stream.then(
        async |item| {
            let a = item.ok().map(|order| {
                OrderResponse::from_order(&state.pool, order)
                    .then(|or| Event::default().data(serde_json::to_string(&or.unwrap()).unwrap()))
            });
            a
        }, /* .map(|order|OrderResponse::from_order(&state.pool, order).map(|order_res| Event::default().data(serde_json::to_string(&order_res.unwrap()).unwrap())
               ,
           ), */
    ); */
    /* let stream = stream::unfold((), move |_| async {
        match rx.recv().await {
            Ok(msg) => Some(
                OrderResponse::from_order(&state.pool, msg)
                    .await
                    .and_then(|or| serde_json::to_string(&or).map_err(ServerError::SerdeJson))
                    .map(|data| Event::default().data(data)),
            ),
            Err(_) => None,
        }
    }); */

    Sse::new(stream).keep_alive(KeepAlive::default())
}

#[derive(Deserialize)]
struct GetByReceiptParams {
    receipt: String,
}
async fn get_by_receipt(
    State(state): State<AppState>,
    _user: User,
    params: Query<GetByReceiptParams>,
) -> Result<Json<OrderResponse>, OrderManagementError> {
    let order = Order::get_by_receipt(&state.pool, &params.receipt)
        .await?
        .ok_or_else(|| OrderManagementError::OrderNotFound)?;
    let res = OrderResponse::from_order(&state.pool, order).await?;

    Ok(Json(res))
}

#[derive(Deserialize)]
struct GetByIdParams {
    id: OrderId,
}
async fn get_by_id(
    State(state): State<AppState>,
    _user: User,
    params: Query<GetByIdParams>,
) -> Result<Json<OrderResponse>, OrderManagementError> {
    let order = Order::get(&state.pool, params.id)
        .await?
        .ok_or_else(|| OrderManagementError::OrderNotFound)?;
    let res = OrderResponse::from_order(&state.pool, order).await?;

    Ok(Json(res))
}

#[derive(Deserialize)]
struct SetServedParams {
    order_id: OrderId,
    new_served: bool,
}
async fn set_served(
    State(state): State<AppState>,
    _user: User,
    params: Query<SetServedParams>,
) -> Result<OkEmptyResponse, OrderManagementError> {
    let mut order = Order::get(&state.pool, params.order_id)
        .await?
        .ok_or_else(|| OrderManagementError::OrderNotFound)?;
    order.set_served(&state.pool, params.new_served).await?;

    Ok(OkEmptyResponse::new())
}

#[derive(Deserialize)]
struct NotifyClient {
    order_id: OrderId,
}
async fn notify_client(
    State(state): State<AppState>,
    _user: User,
    params: Query<NotifyClient>,
) -> Result<OkEmptyResponse, OrderManagementError> {
    let mut order = Order::get(&state.pool, params.order_id)
        .await?
        .ok_or_else(|| OrderManagementError::OrderNotFound)?;

    order
        .notify_client(&state.pool, state.mail_manager.clone())
        .await?;

    Ok(OkEmptyResponse::new())
}
