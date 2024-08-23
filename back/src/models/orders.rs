use crate::errors::{OrderProcessError, ServerError};
use rocket::serde::Deserialize;
use rocket::{
    serde::json::{json, Json, Value},
    State,
};
use sqlx::{types::time::OffsetDateTime, MySql, Pool, Transaction};

use super::stock_manager::StockManager;

#[derive(Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
struct CartElement {
    product_id: u32,
    quantity: u8,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Cart {
    cart: Vec<CartElement>,
}

pub type OrderId = u64;

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Order {
    id: u32,
    payment_intent_id: Option<String>,
    timestamp: OffsetDateTime,
    validated: bool,
    user_email: String,
    receipt: Option<String>,
}
impl Order {
    pub async fn from_id(pool: &Pool<MySql>, id: u32) -> Result<Option<Order>, ServerError> {
        let order_opt = sqlx::query_as!(
            Order,
            "SELECT id, timestamp, validated as \"validated!: bool\", user_email, receipt, payment_intent_id from Orders WHERE id = ?",
            id
        )
        .fetch_optional(pool).await?;
        Ok(order_opt)
    }

    pub async fn set_payment_intent_id(
        &mut self,
        pool: &Pool<MySql>,
        payment_intent_id: String,
    ) -> Result<(), ServerError> {
        let payment_intent_id = payment_intent_id;
        sqlx::query!(
            "UPDATE Orders SET payment_intent_id = ? WHERE id = ?",
            payment_intent_id,
            self.id
        )
        .execute(pool)
        .await?;

        self.payment_intent_id = Some(payment_intent_id);
        Ok(())
    }

    // pub async fn update_status(&mut self, pool: &Pool<MySql>){
    //     let payment_intent_id: PaymentIntentId = match &self.payment_intent_id{
    //         Some(id) => id.parse().unwrap(),
    //         None => return
    //     };
    // }

    pub async fn process_cart_to_order(
        pool: &Pool<MySql>,
        stock_manager: &StockManager,
        order: Cart,
    ) -> Result<OrderId, OrderProcessError> {
        let permit = stock_manager.updating_stock.acquire().await;
        let stock = StockManager::get_all_stocks(pool).await?;
        for cart_element in &order.cart {
            if let Some(stock_for_item) = stock
                .iter()
                .find(|stock_item| stock_item.product_id == cart_element.product_id)
            {
                if stock_for_item.quantity < cart_element.quantity as u32 {
                    return Err(OrderProcessError::NotEnoughStock(
                        stock_for_item.name.clone(),
                        stock_for_item.product_id,
                    ));
                }
            } else {
                return Err(OrderProcessError::ProductNotFound(cart_element.product_id));
            }
        }

        let mut pool_transaction: Transaction<'static, MySql> =
            pool.begin().await.map_err(ServerError::Sqlx)?;
        let order_id = sqlx::query!(
            "INSERT INTO Orders (validated, user_email) VALUES(false, ?)",
            "test@example.com"
        )
        .execute(&mut *pool_transaction)
        .await
        .map_err(ServerError::Sqlx)?
        .last_insert_id();
        for cart_element in &order.cart {
            sqlx::query!(
                "INSERT INTO OrderDetails (order_id, product_id, quantity) VALUES (?, ?, ?)",
                order_id,
                cart_element.product_id,
                cart_element.quantity
            )
            .execute(&mut *pool_transaction)
            .await
            .map_err(ServerError::Sqlx)?;

            sqlx::query!(
                "UPDATE Stock SET quantity = quantity - ? WHERE product_id = ?",
                cart_element.quantity,
                cart_element.product_id
            )
            .execute(&mut *pool_transaction)
            .await
            .map_err(ServerError::Sqlx)?;
        }
        pool_transaction.commit().await.map_err(ServerError::Sqlx)?;

        drop(permit);
        Ok(order_id)
    }
}

// pub struct OrderDetails {
//     id: u32,
//     order_id: u32,
//     product_id: u32,
//     quantity: u32,
// }

impl Order {
    pub async fn get_full_price(&self, pool: &Pool<MySql>) -> Result<i64, ServerError> {
        let total = sqlx::query!(
            "SELECT cast(SUM(price * OrderDetails.quantity) as int) as result from OrderDetails INNER JOIN Stock ON OrderDetails.product_id = Stock.product_id WHERE order_id = ? ;",
            self.id
        ).fetch_one(pool).await.map_err(ServerError::Sqlx)?;

        total
            .result
            .ok_or(ServerError::Sqlx(sqlx::Error::RowNotFound))
    }
}

#[post("/validate_cart", data = "<cart>")]
pub async fn validate_cart(
    pool: &State<Pool<MySql>>,
    stock_manager: &State<StockManager>,
    cart: Json<Cart>,
) -> Result<Json<Value>, OrderProcessError> {
    let order_id = Order::process_cart_to_order(pool, stock_manager, cart.clone().0).await?;
    Ok(Json(json!({"order_id": order_id})))
}
