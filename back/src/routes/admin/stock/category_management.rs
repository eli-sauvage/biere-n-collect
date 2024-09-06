use axum::{routing::post, Router};
use serde::Deserialize;

use crate::{
    admin::user::AdminUser,
    app::product_categories::Category,
    errors::ManageStockError,
    routes::{extractors::CustomQuery as Query, reponders::OkEmptyResponse, AppState},
};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_category).delete(delete_category))
        .route("/edit_name", post(edit_category_name))
}

#[derive(Deserialize)]
struct CreateCategoryParams {
    name: String,
}
async fn create_category(
    _user: AdminUser,
    params: Query<CreateCategoryParams>,
) -> Result<OkEmptyResponse, ManageStockError> {
    Category::create(params.name.to_owned()).await?;

    Ok(OkEmptyResponse::new())
}

#[derive(Deserialize)]
struct EditCategoryParams {
    category_id: u32,
    new_name: String,
}

async fn edit_category_name(
    _user: AdminUser,
    params: Query<EditCategoryParams>,
) -> Result<OkEmptyResponse, ManageStockError> {
    let mut category = match Category::get(params.category_id).await? {
        Some(c) => c,
        None => return Err(ManageStockError::CategoryNotFound(params.category_id)),
    };
    category.set_name(params.new_name.to_owned()).await?;

    Ok(OkEmptyResponse::new())
}

#[derive(Deserialize)]
struct DeleteCategoryParams {
    category_id: u32,
}
async fn delete_category(
    _user: AdminUser,
    params: Query<DeleteCategoryParams>,
) -> Result<OkEmptyResponse, ManageStockError> {
    let category = match Category::get(params.category_id).await? {
        Some(c) => c,
        None => return Err(ManageStockError::CategoryNotFound(params.category_id)),
    };
    category.delete().await?;

    Ok(OkEmptyResponse::new())
}
