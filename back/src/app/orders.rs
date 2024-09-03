use std::time::Duration;

use serde::{Deserialize, Serialize};
use sqlx::{types::time::OffsetDateTime, MySql, Transaction};
use uuid::Uuid;

use super::stripe::api;
use super::stripe::payment_intents::{PaymentIntent, PaymentIntentStatus};
use super::{stock, stripe};
use crate::app::receipt::Receipt;
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
}

pub type OrderId = u64;
#[derive(Serialize)]
pub struct OrderDetailElement {
    pub name: String,
    pub quantity: u8,
    pub subtotal: i32,
}

#[derive(Clone, Debug)]
pub struct Order {
    pub id: OrderId,
    pub timestamp: OffsetDateTime,
    pub user_email: Option<String>,
    pub receipt: Option<Receipt>,
    payment_intent_id: String,
    pub served: bool,
}

impl Order {
    pub async fn get(id: OrderId) -> Result<Option<Order>, ServerError> {
        let order_opt = sqlx::query_as!(
            Order,
            "SELECT id, timestamp, user_email, receipt as \"receipt: Receipt\", payment_intent_id, served as \"served!: bool\" from Orders WHERE id = ?",
            id
        )
        .fetch_optional(db())
        .await?;

        Ok(order_opt)
    }

    pub async fn get_from_client_secret(client_secret: &str) -> Result<Option<Order>, ServerError> {
        let order_opt = sqlx::query_as!(
            Order,
            "SELECT id, timestamp, user_email, receipt as \"receipt: Receipt\", payment_intent_id, served as \"served!: bool\" from Orders WHERE client_secret = ?",
            client_secret
        )
        .fetch_optional(db())
        .await?;
        Ok(order_opt)
    }

    pub async fn get_by_receipt(receipt: &str) -> Result<Option<Order>, ServerError> {
        let order_opt = sqlx::query_as!(
            Order,
            "SELECT id, timestamp, user_email, receipt as \"receipt: Receipt\", payment_intent_id, served as \"served!: bool\" from Orders WHERE receipt = ?",
            receipt
        )
        .fetch_optional(db())
        .await?;
        Ok(order_opt)
    }

    pub async fn set_email(&mut self, email: &str) -> Result<(), ServerError> {
        sqlx::query!(
            "UPDATE Orders SET user_email = ? WHERE id = ?",
            email,
            self.id
        )
        .execute(db())
        .await?;
        self.user_email = Some(email.to_owned());
        Ok(())
    }

    pub async fn set_served(&mut self, served: bool) -> Result<(), ServerError> {
        sqlx::query!("UPDATE Orders SET served = ? WHERE id = ?", served, self.id)
            .execute(db())
            .await?;
        self.served = served;
        Ok(())
    }

    pub async fn get_details(&self) -> Result<Vec<OrderDetailElement>, ServerError> {
        let detail = sqlx::query!(
            "SELECT Stock.name, OrderDetails.quantity, (Stock.price * OrderDetails.quantity) as subtotal FROM OrderDetails INNER JOIN Stock ON OrderDetails.product_id = Stock.product_id WHERE order_id = ? AND OrderDetails.quantity != 0",
            self.id
        ).fetch_all(db()).await?;
        let detail: Vec<OrderDetailElement> = detail
            .into_iter()
            .map(|r| OrderDetailElement {
                name: r.name,
                quantity: r.quantity as u8,
                subtotal: r.subtotal as i32,
            })
            .collect();

        Ok(detail)
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
        self.receipt = Some(Receipt(receipt));
        let self_thread = self.clone();
        tokio::spawn(async move {
            if let Err(e) = self_thread.send_qr().await {
                eprintln!("error while sending receipt mail : {e:?}")
            }
        });
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
            "INSERT INTO Orders (expires, payment_intent_id, client_secret) VALUES (?, ?, ?)",
            expires,
            payment_intent.id,
            payment_intent.client_secret
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
                .fetch_one(db())
                .await?
                .receipt;
            if receipt.is_none() {
                self.mark_as_paid().await?;
            }
        }
        Ok(intent)
    }
    pub async fn get_full_price(&self) -> Result<i32, ServerError> {
        let total = sqlx::query!(
            "SELECT cast(SUM(Stock.price * OrderDetails.quantity) as int) as result from OrderDetails INNER JOIN Stock ON OrderDetails.product_id = Stock.product_id WHERE order_id = ? ;",
            self.id
        ).fetch_one(db()).await.map_err(ServerError::Sqlx)?;

        let res = total
            .result
            .ok_or(ServerError::Sqlx(sqlx::Error::RowNotFound))?;
        Ok(res as i32)
    }
}

pub async fn search_orders(
    email: Option<&str>,
    date_begin: Option<OffsetDateTime>,
    date_end: Option<OffsetDateTime>,
    receipt: Option<&str>,
) -> Result<Vec<Order>, ServerError> {
    let orders = if let Some(date_end) = date_end {
        sqlx::query_as!(
            Order,
            "SELECT id, timestamp, user_email, receipt as \"receipt: Receipt\", payment_intent_id, served as \"served!: bool\"  from Orders
            WHERE user_email LIKE CONCAT('%', ?, '%') AND receipt LIKE CONCAT('%', ?, '%') AND timestamp > ? AND timestamp < ? ORDER BY timestamp DESC",
            email.unwrap_or(""),
            receipt.unwrap_or(""),
            date_begin.unwrap_or(OffsetDateTime::UNIX_EPOCH),
            date_end
        ).fetch_all(db()).await?
    } else {
        sqlx::query_as!(
            Order,
            "SELECT id, timestamp, user_email, receipt as \"receipt: Receipt\", payment_intent_id, served as \"served!: bool\"  from Orders
            WHERE user_email LIKE CONCAT('%', ?, '%') AND receipt LIKE CONCAT('%', ?, '%') AND timestamp > ? ORDER BY timestamp DESC",
            email.unwrap_or(""),
            receipt.unwrap_or(""),
            date_begin.unwrap_or(OffsetDateTime::UNIX_EPOCH),
        ).fetch_all(db()).await?
    };
    Ok(orders)
}
