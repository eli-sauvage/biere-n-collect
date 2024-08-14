use crate::errors::{Error, OrderError};
use rocket::serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};
use tokio::sync::Semaphore;

//TODO: dynamique, pour l'instant => que des pintes
const VOLUME_PER_ITEM: f32 = 0.5;

#[derive(Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
struct CartElement {
    product_id: u32,
    quantity: u8,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct IncomingOrder {
    cart: Vec<CartElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Stock {
    pub name: String,
    pub stock: f32,
    pub product_id: u32,
    pub price: i32,
}

pub async fn get_all_stocks(pool: &Pool<MySql>) -> Result<Vec<Stock>, Error> {
    let products = sqlx::query_as!(
        Stock,
        "SELECT name, stock, product_id, price FROM Stocks
                INNER JOIN ProductTypes ON Stocks.product_id = ProductTypes.id"
    )
    .fetch_all(pool)
    .await
    .map_err(Error::Sqlx)?;

    Ok(products)
}

#[derive(Serialize, Deserialize)]
pub struct CartValidationResponse {
    pub order_id: u64,
}
pub struct StockManager {
    updating_stock: Semaphore,
}
impl StockManager {
    pub fn new() -> StockManager {
        StockManager {
            updating_stock: Semaphore::new(1),
        }
    }

    pub async fn process_order(
        &self,
        pool: &Pool<MySql>,
        order: IncomingOrder,
    ) -> Result<u64, Error> {
        let permit = self.updating_stock.acquire().await;
        let stock = get_all_stocks(pool).await?;
        for cart_element in &order.cart {
            if let Some(stock_for_item) = stock
                .iter()
                .find(|stock_item| stock_item.product_id == cart_element.product_id)
            {
                if stock_for_item.stock < cart_element.quantity as f32 * VOLUME_PER_ITEM {
                    return Err(Error::Order(OrderError::NotEnoughStock(
                        stock_for_item.name.clone(),
                        stock_for_item.product_id,
                    )));
                }
            } else {
                return Err(Error::Order(OrderError::ProductNotFound(
                    cart_element.product_id,
                )));
            }
        }

        // let total_price = order.cart.iter().fold(0.0, |acc, cart_elem| {
        //     if let Some(stock_for_item) = stock
        //         .iter()
        //         .find(|stock_item| stock_item.product_id == cart_elem.product_id)
        //     {
        //         acc + stock_for_item.price * cart_elem.quantity as f32
        //     } else {
        //         acc
        //     }
        // });

        //the cart is now validated (correct id and quantities)
        let mut pool_transaction = pool.begin().await?;
        let order_id = sqlx::query!(
            "INSERT INTO Orders (validated, user_email) VALUES(false, ?)",
            "test@example.com"
        )
        .execute(&mut *pool_transaction)
        .await?
        .last_insert_id();
        for cart_element in &order.cart {
            sqlx::query!(
                "INSERT INTO OrderDetails (order_id, product_id, quantity) VALUES (?, ?, ?)",
                order_id,
                cart_element.product_id,
                cart_element.quantity
            )
            .execute(&mut *pool_transaction)
            .await?;

            sqlx::query!(
                "UPDATE Stocks SET stock = stock - ? WHERE product_id = ?",
                cart_element.quantity,
                cart_element.product_id
            )
            .execute(&mut *pool_transaction)
            .await?;
        }
        pool_transaction.commit().await?;

        drop(permit);
        Ok(order_id)
    }
}
