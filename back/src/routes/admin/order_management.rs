use axum::{routing::get, Json, Router};
use serde::{Serialize, Serializer};
use sqlx::types::time::OffsetDateTime;

use crate::{
    admin::user::User,
    app::orders::{self, OrderDetailElement},
    errors::OrderManagementError,
    routes::AppState,
};

pub fn get_router() -> Router<AppState> {
    Router::new().route("/get_all", get(get_all_orders))
}

#[derive(Serialize)]
struct OrderResponse {
    receipt: Option<String>,
    served: bool,
    #[serde(serialize_with = "serialize_time")]
    timestamp: OffsetDateTime,
    user_email: Option<String>,
    detail: Vec<OrderDetailElement>,
    total_price: i32,
}

fn serialize_time<S: Serializer>(dt: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error> {
    let time = dt.unix_timestamp() * 1000;
    serializer.serialize_i64(time)
}

async fn get_all_orders(_user: User) -> Result<Json<Vec<OrderResponse>>, OrderManagementError> {
    let orders = orders::get_all_orders().await?;
    let mut res: Vec<OrderResponse> = vec![];
    for order in orders {
        let details = order
            .get_details()
            .await
            .map_err(OrderManagementError::ServerError)?;
        let total_price = order.get_full_price().await?;
        res.push(OrderResponse {
            receipt: order.receipt.as_deref().cloned(),
            served: order.served,
            timestamp: order.timestamp,
            user_email: order.user_email,
            detail: details,
            total_price,
        })
    }
    Ok(Json(res))
}
