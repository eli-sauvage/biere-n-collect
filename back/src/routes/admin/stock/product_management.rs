use axum::{
    routing::{patch, post},
    Router,
};
use serde::Deserialize;

use crate::{
    admin::user::AdminUser,
    app::products::{self, MoveDirection, Product},
    errors::ManageStockError,
    routes::{extractors::CustomQuery as Query, reponders::OkEmptyResponse, AppState},
    utils::deserialize_empty_as_none,
};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(insert_product)
                .patch(edit_product)
                .delete(delete_product),
        )
        .route("/move", patch(move_product))
        .route("/add_variation", post(add_variation))
        .route("/remove_variation", post(remove_variation))
}

#[derive(Deserialize)]
struct InsertProductParams {
    name: String,
    description: String,
    stock_quantity: i32,
    available_to_order: bool,
}
async fn insert_product(
    _user: AdminUser,
    params: Query<InsertProductParams>,
) -> Result<OkEmptyResponse, ManageStockError> {

    products::Product::create(
        params.name.clone(),
        params.description.clone(),
        params.stock_quantity,
        params.available_to_order,
    )
    .await?;

    Ok(OkEmptyResponse::new())
}

#[derive(Deserialize)]
struct EditProductParams {
    product_id: u32,
    #[serde(default, deserialize_with = "deserialize_empty_as_none")]
    new_name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_empty_as_none")]
    new_description: Option<String>,
    #[serde(default, deserialize_with = "deserialize_empty_as_none")]
    new_stock_quantity: Option<i32>,
    #[serde(default, deserialize_with = "deserialize_empty_as_none")]
    new_available_to_order: Option<bool>,
}

async fn edit_product(
    _user: AdminUser,
    params: Query<EditProductParams>,
) -> Result<OkEmptyResponse, ManageStockError> {
    let mut product = match Product::get(params.product_id).await? {
        Some(p) => p,
        None => return Err(ManageStockError::ProductNotFound(params.product_id)),
    };

    if let Some(new_name) = &params.new_name {
        product.set_name(new_name.to_owned()).await?;
    }
    if let Some(new_description) = &params.new_description {
        product.set_description(new_description.to_owned()).await?;
    }
    if let Some(new_stock_quantity) = params.new_stock_quantity {
        product.set_stock_quantity(new_stock_quantity).await?;
    }
    if let Some(new_available_to_order) = params.new_available_to_order {
        product
            .set_available_to_order(new_available_to_order)
            .await?;
    }

    Ok(OkEmptyResponse::new())
}

#[derive(Deserialize)]
struct DeleteProductParams {
    product_id: u32,
}
async fn delete_product(
    _user: AdminUser,
    params: Query<DeleteProductParams>,
) -> Result<OkEmptyResponse, ManageStockError> {
    let product = match Product::get(params.product_id).await? {
        Some(p) => p,
        None => return Err(ManageStockError::ProductNotFound(params.product_id)),
    };
    product.delete().await?;

    Ok(OkEmptyResponse::new())
}

#[derive(Deserialize)]
struct MoveProductParams {
    product_id: u32,
    direction: MoveDirection,
}
async fn move_product(
    _user: AdminUser,
    params: Query<MoveProductParams>,
) -> Result<OkEmptyResponse, ManageStockError> {
    let mut product = match Product::get(params.product_id).await? {
        Some(p) => p,
        None => return Err(ManageStockError::ProductNotFound(params.product_id)),
    };
    product.move_product(params.direction).await?;

    Ok(OkEmptyResponse::new())
}

#[derive(Deserialize)]
struct AddVariationParams {
    product_id: u32,
    name: String,
    price_ht: i32,
    tva: f32,
    volume: f32,
    available_to_order: bool,
}
async fn add_variation(
    _user: AdminUser,
    params: Query<AddVariationParams>,
) -> Result<OkEmptyResponse, ManageStockError> {
    let mut product = match Product::get(params.product_id).await? {
        Some(p) => p,
        None => return Err(ManageStockError::ProductNotFound(params.product_id)),
    };

    product
        .add_variation(
            params.name.to_owned(),
            params.price_ht,
            params.tva,
            params.volume,
            params.available_to_order,
        )
        .await?;

    Ok(OkEmptyResponse::new())
}

#[derive(Deserialize)]
struct RemoveVariationParams {
    product_id: u32,
    variation_id: u32,
}
async fn remove_variation(
    _user: AdminUser,
    params: Query<RemoveVariationParams>,
) -> Result<OkEmptyResponse, ManageStockError> {
    let mut product = match Product::get(params.product_id).await? {
        Some(p) => p,
        None => return Err(ManageStockError::ProductNotFound(params.product_id)),
    };
    product.delete_variation(params.variation_id).await?;

    Ok(OkEmptyResponse::new())
    }
