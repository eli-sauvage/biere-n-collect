use crate::{
    errors::{ManageStockError, ServerError},
    users::user::{AdminUser, ErrorMsg},
};
use rocket::{
    serde::json::{json, Json, Value},
    State,
};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool, Transaction};
use tokio::sync::Semaphore;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Stock {
    pub name: String,
    pub quantity: u32,
    pub product_id: u32,
    pub price: i32,
    pub available: bool,
}

enum MoveDirection {
    Up,
    Down,
}

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
            "SELECT name, quantity, product_id, price, available as \"available!: bool\" FROM Stock ORDER BY position"
        )
        .fetch_all(pool)
        .await
        .map_err(ServerError::Sqlx)?;

        Ok(products)
    }

    pub async fn update_stock(
        &self,
        pool: &Pool<MySql>,
        new_stock: Stock,
    ) -> Result<(), ServerError> {
        let permit = self.updating_stock.acquire().await;
        sqlx::query!(
            "UPDATE Stock SET quantity = ?, price = ?, available = ?, name = ? WHERE product_id = ?",
            new_stock.quantity,
            new_stock.price,
            new_stock.available,
            new_stock.name,
            new_stock.product_id
        )
        .execute(pool)
        .await?;
        drop(permit);
        Ok(())
    }

    async fn insert_stock(
        &self,
        pool: &Pool<MySql>,
        name: String,
        price: u32,
        quantity: u32,
        available: bool,
    ) -> Result<(), ServerError> {
        let permit = self.updating_stock.acquire().await;
        let mut pool_transaction: Transaction<'static, MySql> =
            pool.begin().await.map_err(ServerError::Sqlx)?;
        sqlx::query!("UPDATE Stock SET position = position + 1")
            .execute(&mut *pool_transaction)
            .await?;

        sqlx::query!(
            "INSERT INTO Stock (name, price, quantity, available, position) VALUES (?, ?, ?, ?, 0)",
            name,
            price,
            quantity,
            available
        )
        .execute(&mut *pool_transaction)
        .await?;
        drop(permit);
        pool_transaction.commit().await?;
        Ok(())
    }

    async fn delete_stock(&self, pool: &Pool<MySql>, product_id: u32) -> Result<(), ServerError> {
        let permit = self.updating_stock.acquire().await;
        let old_position = sqlx::query!(
            "SELECT position from Stock WHERE product_id = ?",
            product_id
        )
        .fetch_one(pool)
        .await?
        .position;

        let mut pool_transaction: Transaction<'static, MySql> =
            pool.begin().await.map_err(ServerError::Sqlx)?;
        sqlx::query!("DELETE FROM Stock WHERE product_id = ?", product_id)
            .execute(&mut *pool_transaction)
            .await?;

        sqlx::query!(
            "UPDATE Stock SET position = position - 1 WHERE position > ?",
            old_position
        )
        .execute(&mut *pool_transaction)
        .await?;
        pool_transaction.commit().await?;
        drop(permit);
        Ok(())
    }

    async fn move_stock(
        &self,
        pool: &Pool<MySql>,
        direction: MoveDirection,
        product_id: u32,
    ) -> Result<(), ManageStockError> {
        let permit = self.updating_stock.acquire().await;
        let current_position = sqlx::query!(
            "SELECT position from Stock WHERE product_id = ?",
            product_id
        )
        .fetch_one(pool)
        .await
        .map_err(ServerError::Sqlx)?
        .position;

        let max_pos = sqlx::query!("SELECT Max(position) as max_pos from Stock")
            .fetch_one(pool)
            .await
            .map_err(ServerError::Sqlx)?
            .max_pos
            .unwrap_or(0);

        let new_position = match direction {
            MoveDirection::Up => {
                if current_position == 0 {
                    return Err(ManageStockError::CannotMoveUp(product_id));
                }

                current_position - 1
            }
            MoveDirection::Down => {
                if current_position == max_pos {
                    return Err(ManageStockError::CannotMoveDown(product_id));
                }
                current_position + 1
            }
        };

        let mut pool_transaction: Transaction<'static, MySql> =
            pool.begin().await.map_err(ServerError::Sqlx)?;

        sqlx::query!(
            "UPDATE Stock SET position = ? WHERE position = ?",
            current_position,
            new_position
        )
        .execute(&mut *pool_transaction)
        .await
        .map_err(ServerError::Sqlx)?;
        sqlx::query!(
            "UPDATE Stock SET position = ? WHERE product_id = ?",
            new_position,
            product_id
        )
        .execute(&mut *pool_transaction)
        .await
        .map_err(ServerError::Sqlx)?;

        pool_transaction.commit().await.map_err(ServerError::Sqlx)?;
        drop(permit);
        Ok(())
    }
}

#[get("/get")]
pub async fn get_stocks(pool: &State<Pool<MySql>>) -> Result<Json<Vec<Stock>>, ServerError> {
    let p: Vec<Stock> = StockManager::get_all_stocks(pool)
        .await?
        .into_iter()
        .filter(|stock| stock.available)
        .collect();
    Ok(Json(p))
}

#[get("/get_all")]
pub async fn get_all_stocks(
    user: Result<AdminUser, ErrorMsg>,
    pool: &State<Pool<MySql>>,
) -> Result<Json<Vec<Stock>>, ManageStockError> {
    if let Err(e) = user {
        return Err(ManageStockError::NotAdmin(e));
    }

    let p = StockManager::get_all_stocks(pool).await?;
    Ok(Json(p))
}

#[put("/", data = "<stock>")]
pub async fn update_stock(
    user: Result<AdminUser, ErrorMsg>,
    pool: &State<Pool<MySql>>,
    stock_manager: &State<StockManager>,
    stock: Json<Stock>,
) -> Result<Json<Value>, ManageStockError> {
    if let Err(e) = user {
        return Err(ManageStockError::NotAdmin(e));
    }
    let new_stock = stock.clone().0;
    if !StockManager::get_all_stocks(pool)
        .await?
        .into_iter()
        .any(|stock| stock.product_id == new_stock.product_id)
    {
        return Err(ManageStockError::StockNotFound(new_stock.product_id));
    }

    stock_manager.update_stock(pool, new_stock).await?;

    Ok(Json(json!({"success": true})))
}

#[post("/?<name>&<price>&<quantity>&<available>")]
pub async fn insert_stock(
    user: Result<AdminUser, ErrorMsg>,
    pool: &State<Pool<MySql>>,
    stock_manager: &State<StockManager>,
    name: String,
    price: u32,
    quantity: u32,
    available: bool,
) -> Result<Json<Value>, ManageStockError> {
    if let Err(e) = user {
        return Err(ManageStockError::NotAdmin(e));
    }

    stock_manager
        .insert_stock(pool, name, price, quantity, available)
        .await?;

    Ok(Json(json!({"success": true})))
}

#[delete("/?<product_id>")]
pub async fn delete_stock(
    user: Result<AdminUser, ErrorMsg>,
    pool: &State<Pool<MySql>>,
    stock_manager: &State<StockManager>,
    product_id: u32,
) -> Result<Json<Value>, ManageStockError> {
    if let Err(e) = user {
        return Err(ManageStockError::NotAdmin(e));
    }

    if !StockManager::get_all_stocks(pool)
        .await?
        .into_iter()
        .any(|stock| stock.product_id == product_id)
    {
        return Err(ManageStockError::StockNotFound(product_id));
    }

    stock_manager.delete_stock(pool, product_id).await?;

    Ok(Json(json!({"success": true})))
}

#[patch("/move?<product_id>&<direction>")]
pub async fn move_stock(
    user: Result<AdminUser, ErrorMsg>,
    pool: &State<Pool<MySql>>,
    stock_manager: &State<StockManager>,
    product_id: u32,
    direction: String,
) -> Result<Json<Value>, ManageStockError> {
    if let Err(e) = user {
        return Err(ManageStockError::NotAdmin(e));
    }

    if !StockManager::get_all_stocks(pool)
        .await?
        .into_iter()
        .any(|stock| stock.product_id == product_id)
    {
        return Err(ManageStockError::StockNotFound(product_id));
    }

    let direction = match direction.as_str() {
        "up" => MoveDirection::Up,
        "down" => MoveDirection::Down,
        _ => return Err(ManageStockError::DirectionDoesNotExist(direction)),
    };

    stock_manager
        .move_stock(pool, direction, product_id)
        .await?;

    Ok(Json(json!({"success": true})))
}
