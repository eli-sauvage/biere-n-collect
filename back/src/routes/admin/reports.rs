use axum::{extract::State, routing::get, Json, Router};
use serde::Deserialize;
use sqlx::types::time::OffsetDateTime;

use crate::{
    admin::{
        bar_management,
        report::{process_orders_to_report, Report},
        user::AdminUser,
    },
    app::orders,
    errors::{OrderManagementError, ServerError},
    routes::{extractors::CustomQuery as Query, AppState},
};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/get_bar_openings", get(get_bar_openings))
        .route("/", get(get_report))
}

async fn get_bar_openings(
    State(state): State<AppState>,
    _user: AdminUser,
) -> Result<Json<Vec<bar_management::BarOpening>>, ServerError> {
    let openings = bar_management::get_bar_openings(&state.pool).await?;
    Ok(Json(openings))
}

#[derive(Deserialize)]
struct GetReportQuery {
    begin: i64,
    end: i64,
}
async fn get_report(
    State(state): State<AppState>,
    _user: AdminUser,
    params: Query<GetReportQuery>,
) -> Result<Json<Report>, OrderManagementError> {
    let begin = OffsetDateTime::from_unix_timestamp(params.begin / 1000)
        .map_err(|_| OrderManagementError::InvalidDate)?;
    let end = OffsetDateTime::from_unix_timestamp(params.end / 1000)
        .map_err(|_| OrderManagementError::InvalidDate)?;
    let orders = orders::search_orders(&state.pool, None, Some(begin), Some(end), None).await?;
    let report = process_orders_to_report(&state.pool, orders).await?;
    Ok(Json(report))
}
