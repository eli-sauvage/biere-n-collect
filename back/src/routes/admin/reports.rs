use axum::{routing::get, Json, Router};
use serde::Deserialize;
use sqlx::types::time::OffsetDateTime;

use crate::{
    admin::bar_management,
    admin::user::AdminUser,
    app::orders,
    errors::{ServerError, OrderManagementError},
    routes::{AppState, admin::order_management::OrderResponse},
    routes::extractors::CustomQuery as Query
};

pub fn get_router() -> Router<AppState> {
    Router::new().route("/get_bar_openings", get(get_bar_openings))
    .route("/", get(get_report))
}

async fn get_bar_openings(_user: AdminUser) -> Result<Json<Vec<bar_management::BarOpening>>, ServerError> {
    let openings = bar_management::get_bar_openings().await?;
    Ok(Json(openings))
}

#[derive(Deserialize)]
struct GetReportQuery{
    begin: i64,
    end: i64
}
async fn get_report(_user: AdminUser, params: Query<GetReportQuery>) -> Result<Json<Vec<OrderResponse>>, OrderManagementError>{
    let begin = OffsetDateTime::from_unix_timestamp(params.begin / 1000).map_err(|_|OrderManagementError::InvalidDate)?;
    let end = OffsetDateTime::from_unix_timestamp(params.end / 1000).map_err(|_|OrderManagementError::InvalidDate)?;
    let mut joins = tokio::task::JoinSet::new();
    orders::search_orders(None,Some(begin), Some(end), None)
        .await?
        .into_iter()
        .for_each(|order|{
            joins.spawn(OrderResponse::from_order(order));
        });
    let orders = joins.join_all().await.into_iter().collect::<Result<Vec<_>, _>>()?;
    Ok(Json(orders))
}
