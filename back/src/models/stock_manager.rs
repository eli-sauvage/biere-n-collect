use crate::{
    errors::{ServerError, UpdateStockError},
    users::user::{Role, User},
};
use rocket::{
    serde::json::{json, Json, Value},
    State,
};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};
use tokio::sync::Semaphore;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Stock {
    pub name: String,
    pub stock: f32,
    pub product_id: u32,
    pub price: i32,
}

// #[derive(Serialize, Deserialize)]
// pub struct CartValidationResponse {
//     pub order_id: OrderId,
// }
pub struct StockManager {
    pub updating_stock: Semaphore,
}
impl StockManager {
    pub fn new() -> StockManager {
        StockManager {
            updating_stock: Semaphore::new(1),
        }
    }
    pub async fn get_all_stocks(pool: &Pool<MySql>) -> Result<Vec<Stock>, ServerError> {
        let products = sqlx::query_as!(
            Stock,
            "SELECT name, stock, product_id, price FROM Stocks
                INNER JOIN ProductTypes ON Stocks.product_id = ProductTypes.id"
        )
        .fetch_all(pool)
        .await
        .map_err(ServerError::Sqlx)?;

        Ok(products)
    }

    pub async fn update_stock(
        &self,
        pool: &Pool<MySql>,
        product_id: u32,
        new_quantity: u32,
    ) -> Result<(), ServerError> {
        let permit = self.updating_stock.acquire().await;
        sqlx::query!(
            "UPDATE Stocks SET stock = ? WHERE product_id = ?",
            new_quantity,
            product_id
        )
        .execute(pool)
        .await?;
        drop(permit);
        Ok(())
    }
}

#[post("/update?<product_id>&<new_quantity>")]
pub async fn update_stock(
    user: User,
    pool: &State<Pool<MySql>>,
    stock_manager: &State<StockManager>,
    product_id: u32,
    new_quantity: u32,
) -> Result<Json<Value>, UpdateStockError> {
    if user.role != Role::Admin {
        return Err(UpdateStockError::NotAdmin(user.email.clone()));
    }

    stock_manager
        .update_stock(pool, product_id, new_quantity)
        .await?;
    Ok(Json(json!({"success": true})))
}

#[get("/get")]
pub async fn get_stocks(pool: &State<Pool<MySql>>) -> Result<Json<Vec<Stock>>, ServerError> {
    let p = StockManager::get_all_stocks(pool).await?;
    Ok(Json(p))
}
