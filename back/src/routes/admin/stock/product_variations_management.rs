use axum::{routing::patch, Router};
use serde::Deserialize;

use crate::{
    admin::user::AdminUser,
    app::product_variations::Variation,
    errors::ManageStockError,
    routes::{extractors::CustomQuery as Query, reponders::OkEmptyResponse, AppState},
    utils::deserialize_empty_as_none,
};

pub fn get_router() -> Router<AppState> {
    Router::new().route("/edit", patch(edit_variation))
}

#[derive(Deserialize)]
struct EditVariationsParams {
    variation_id: u32,
    #[serde(default, deserialize_with = "deserialize_empty_as_none")]
    new_name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_empty_as_none")]
    new_price: Option<i32>,
    #[serde(default, deserialize_with = "deserialize_empty_as_none")]
    new_volume: Option<f32>,
    #[serde(default, deserialize_with = "deserialize_empty_as_none")]
    new_available_to_order: Option<bool>,
}

async fn edit_variation(
    _user: AdminUser,
    params: Query<EditVariationsParams>,
) -> Result<OkEmptyResponse, ManageStockError> {
    let mut variation = match Variation::get(params.variation_id).await? {
        Some(c) => c,
        None => return Err(ManageStockError::VariationNotFound(params.variation_id)),
    };

    if let Some(new_name) = &params.new_name {
        variation.set_name(new_name.to_owned()).await?;
    }

    if let Some(new_price) = params.new_price {
        variation.set_price_ht(new_price).await?;
    }

    if let Some(new_volume) = params.new_volume {
        variation.set_volume(new_volume).await?;
    }

    if let Some(new_available_to_order) = params.new_available_to_order {
        variation
            .set_available_to_order(new_available_to_order)
            .await?;
    }

    Ok(OkEmptyResponse::new())
}
