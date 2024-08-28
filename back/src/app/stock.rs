use crate::{db, errors::{ManageStockError, ServerError}};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Transaction};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Stock {
    pub name: String,
    pub quantity: i32,
    pub product_id: u32,
    pub price: i32,
    pub available: bool,
}

#[derive(Deserialize, Copy, Clone)]
#[serde(rename_all="lowercase")]
pub enum MoveDirection {
    Up,
    Down,
}

pub async fn get_all_stocks() -> Result<Vec<Stock>, ServerError> {
    let products = sqlx::query_as!(
            Stock,
            "SELECT name, quantity, product_id, price, available as \"available!: bool\" FROM Stock ORDER BY position"
        )
        .fetch_all(db())
        .await
        .map_err(ServerError::Sqlx)?;

    Ok(products)
}

pub async fn update_stock(new_stock: Stock) -> Result<(), ServerError> {
    sqlx::query!(
        "UPDATE Stock SET quantity = ?, price = ?, available = ?, name = ? WHERE product_id = ?",
        new_stock.quantity,
        new_stock.price,
        new_stock.available,
        new_stock.name,
        new_stock.product_id
    )
    .execute(db())
    .await?;
    Ok(())
}

pub async fn insert_stock(
    name: &str,
    price: u32,
    quantity: u32,
    available: bool,
) -> Result<(), ServerError> {
    let mut pool_transaction: Transaction<'static, MySql> =
        db().begin().await.map_err(ServerError::Sqlx)?;
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
    pool_transaction.commit().await?;
    Ok(())
}

pub async fn delete_stock(product_id: u32) -> Result<(), ServerError> {
    let old_position = sqlx::query!(
        "SELECT position from Stock WHERE product_id = ?",
        product_id
    )
    .fetch_one(db())
    .await?
    .position;

    let mut pool_transaction: Transaction<'static, MySql> =
        db().begin().await.map_err(ServerError::Sqlx)?;
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
    Ok(())
}

pub async fn move_stock(
    direction: MoveDirection,
    product_id: u32,
) -> Result<(), ManageStockError> {
    let current_position = sqlx::query!(
        "SELECT position from Stock WHERE product_id = ?",
        product_id
    )
    .fetch_one(db())
    .await
    .map_err(ServerError::Sqlx)?
    .position;

    let max_pos = sqlx::query!("SELECT Max(position) as max_pos from Stock")
        .fetch_one(db())
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
        db().begin().await.map_err(ServerError::Sqlx)?;

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
    Ok(())
}
