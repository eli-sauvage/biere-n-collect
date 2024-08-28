use std::time::Duration;

use serde::{Deserialize, Serialize, Serializer};
use sqlx::{types::time::OffsetDateTime, MySql, Transaction};
use uuid::Uuid;

use super::stripe::api;
use super::stripe::payment_intents::{PaymentIntent, PaymentIntentStatus};
use super::{stock, stripe};
use crate::db;
use crate::errors::{OrderProcessError, ServerError};

//time for an order before it gets deleted if unpaid
const ORDER_DURATION_MINUTES: u64 = 10;

#[derive(Deserialize, Clone, Debug)]
struct CartElement {
    product_id: u32,
    quantity: u8,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Cart {
    elements: Vec<CartElement>,
    email: String,
}

pub type OrderId = u64;
pub type Receipt = String;

#[derive(Clone, Debug, Serialize)]
pub struct Order {
    pub id: OrderId,
    #[serde(serialize_with = "serialize_time")]
    timestamp: OffsetDateTime,
    user_email: String,
    receipt: Option<Receipt>,
    payment_intent_id: String,
}
fn serialize_time<S: Serializer>(dt: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error> {
    let time = dt.to_string();
    serializer.serialize_str(&time)
}

impl Order {
    pub async fn get(id: OrderId) -> Result<Option<Order>, ServerError> {
        let order_opt = sqlx::query_as!(
            Order,
            "SELECT id, timestamp, user_email, receipt, payment_intent_id  from Orders WHERE id = ?",
            id
        )
        .fetch_optional(db())
        .await?;

        Ok(order_opt)
    }

    async fn mark_as_paid(&mut self) -> Result<(), ServerError> {
        let receipt = Uuid::new_v4().to_string();
        sqlx::query!(
            "UPDATE Orders SET receipt = ? WHERE id = ?",
            receipt,
            self.id
        )
        .execute(db())
        .await?;
        self.receipt = Some(receipt);

        let detail: Vec<(u32, u32)> = sqlx::query!(
            "SELECT product_id, quantity FROM OrderDetails WHERE order_id = ?",
            self.id
        )
        .fetch_all(db())
        .await?
        .into_iter()
        .map(|r| (r.product_id, r.quantity))
        .collect();
        let mut pool_transaction: Transaction<'static, MySql> =
            db().begin().await.map_err(ServerError::Sqlx)?;

        for (product_id, quantity) in detail {
            sqlx::query!(
                "UPDATE Stock SET quantity = quantity - ? WHERE product_id = ?",
                quantity,
                product_id
            )
            .execute(&mut *pool_transaction)
            .await
            .map_err(ServerError::Sqlx)?;
        }

        pool_transaction.commit().await.map_err(ServerError::Sqlx)?;
        Ok(())
    }

    pub async fn generate_from_cart(cart: Cart) -> Result<OrderId, OrderProcessError> {
        let stock = stock::get_all_stocks().await?;
        let mut total_price: i32 = 0;
        for cart_element in &cart.elements {
            if let Some(stock_for_item) = stock
                .iter()
                .find(|stock_item| stock_item.product_id == cart_element.product_id)
            {
                if stock_for_item.quantity >= cart_element.quantity as i32 {
                    total_price += stock_for_item.price * cart_element.quantity as i32;
                } else {
                    return Err(OrderProcessError::NotEnoughStock(
                        stock_for_item.name.clone(),
                        stock_for_item.product_id,
                    ));
                }
            } else {
                return Err(OrderProcessError::ProductNotFound(cart_element.product_id));
            }
        }

        let payment_intent = stripe::api::create_payment_intent(total_price as i64).await?;
        let expires = OffsetDateTime::now_utc() + Duration::from_secs(60 * ORDER_DURATION_MINUTES);
        let order_id = sqlx::query!(
            "INSERT INTO Orders (user_email, expires, payment_intent_id) VALUES (?, ?, ?)",
            cart.email,
            expires,
            payment_intent.id
        )
        .execute(db())
        .await
        .map_err(ServerError::Sqlx)?
        .last_insert_id();
        for cart_element in &cart.elements {
            sqlx::query!(
                "INSERT INTO OrderDetails (order_id, product_id, quantity) VALUES (?, ?, ?)",
                order_id,
                cart_element.product_id,
                cart_element.quantity
            )
            .execute(db())
            .await
            .map_err(ServerError::Sqlx)?;
        }

        Ok(order_id)
    }
    pub async fn get_payment_intent(&mut self) -> Result<PaymentIntent, ServerError> {
        let intent = api::fetch_payment_intent(&self.payment_intent_id).await?;
        if intent.status == PaymentIntentStatus::Succeeded {
            let receipt = sqlx::query!("SELECT receipt FROM Orders WHERE id = ?", self.id)
                .fetch_optional(db())
                .await?;
            if receipt.is_none() {
                self.mark_as_paid().await?;
            }
        }
        Ok(intent)
    }
    // pub async fn get_full_price(&self, pool: &Pool<MySql>) -> Result<i64, ServerError> {
    //     let total = sqlx::query!(
    //         "SELECT cast(SUM(price * OrderDetails.quantity) as int) as result from OrderDetails INNER JOIN Stock ON OrderDetails.product_id = Stock.product_id WHERE order_id = ? ;",
    //         self.id
    //     ).fetch_one(pool).await.map_err(ServerError::Sqlx)?;

    //     total
    //         .result
    //         .ok_or(ServerError::Sqlx(sqlx::Error::RowNotFound))
    // }
}

pub async fn get_all_orders() -> Result<Vec<Order>, ServerError> {
    let orders = sqlx::query_as!(
        Order,
        "SELECT id, timestamp, user_email, receipt, payment_intent_id from Orders"
    )
    .fetch_all(db())
    .await?;
    Ok(orders)
}
