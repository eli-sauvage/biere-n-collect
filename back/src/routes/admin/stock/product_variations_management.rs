use axum::{extract::State, routing::patch, Router};
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
    new_price_ht: Option<i32>,
    #[serde(default, deserialize_with = "deserialize_empty_as_none")]
    new_tva: Option<f32>,
    #[serde(default, deserialize_with = "deserialize_empty_as_none")]
    new_volume: Option<f32>,
    #[serde(default, deserialize_with = "deserialize_empty_as_none")]
    new_available_to_order: Option<bool>,
}

async fn edit_variation(
    State(state): State<AppState>,
    _user: AdminUser,
    params: Query<EditVariationsParams>,
) -> Result<OkEmptyResponse, ManageStockError> {
    let mut variation = match Variation::get(&state.pool, params.variation_id).await? {
        Some(c) => c,
        None => return Err(ManageStockError::VariationNotFound(params.variation_id)),
    };

    if let Some(new_name) = &params.new_name {
        variation.set_name(&state.pool, new_name.to_owned()).await?;
    }

    if let Some(new_price) = params.new_price_ht {
        variation.set_price_ht(&state.pool, new_price).await?;
    }

    if let Some(new_tva) = params.new_tva {
        variation.set_tva(&state.pool, new_tva).await?;
    }

    if let Some(new_volume) = params.new_volume {
        variation.set_volume(&state.pool, new_volume).await?;
    }

    if let Some(new_available_to_order) = params.new_available_to_order {
        variation
            .set_available_to_order(&state.pool, new_available_to_order)
            .await?;
    }

    Ok(OkEmptyResponse::new())
}
