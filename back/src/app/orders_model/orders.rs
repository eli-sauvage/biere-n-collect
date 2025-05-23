use std::{sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};
use sqlx::{types::time::OffsetDateTime, Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

use crate::{
    app::{
        orders_model::mail,
        product_variations::Variation,
        products,
        receipt::Receipt,
        stripe::{
            self,
            payment_intents::{PaymentIntent, PaymentIntentStatus},
        },
    },
    errors::{OrderManagementError, OrderProcessError, ServerError},
    mail_manager::MailManager,
};

const ORDER_DURATION: Duration = Duration::from_secs(10 * 60 * 60);

#[derive(Deserialize, Clone, Debug)]
pub struct CartElement {
    pub variation_id: u32,
    pub quantity: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Cart {
    pub elements: Vec<CartElement>,
}

pub type OrderId = u32;
#[derive(Serialize)]
pub struct OrderDetailElement {
    pub item_name: String,
    pub quantity: u32,
    pub tva: f32,
    pub subtotal_ht: i32,
    pub subtotal_ttc: i32,
}

#[derive(Clone, Debug)]
pub struct Order {
    pub id: OrderId,
    pub timestamp: OffsetDateTime,
    pub user_email: Option<String>,
    pub receipt: Option<Receipt>,
    pub payment_intent_id: String,
    pub served: bool,
}

impl Order {
    pub async fn get(pool: &SqlitePool, id: OrderId) -> Result<Option<Order>, ServerError> {
        cancel_expired_orders(pool);
        let order_opt = sqlx::query_as!(
            Order,
            "SELECT id as \"id: u32\", timestamp, user_email, receipt as \"receipt: Receipt\", payment_intent_id, served as \"served!: bool\" from Orders WHERE id = ? AND (expires > CURRENT_TIMESTAMP OR expires IS NULL)",
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(order_opt)
    }

    pub async fn get_from_client_secret(
        pool: &SqlitePool,
        client_secret: &str,
    ) -> Result<Option<Order>, ServerError> {
        let order_opt = sqlx::query_as!(
           Order,
            "SELECT id as \"id: u32\", timestamp, user_email, receipt as \"receipt: Receipt\", payment_intent_id, served as \"served!: bool\" from Orders WHERE client_secret = ? AND (expires > CURRENT_TIMESTAMP OR expires IS NULL)",
            client_secret
        )
        .fetch_optional(pool)
        .await?;
        Ok(order_opt)
    }

    pub async fn get_by_receipt(
        pool: &SqlitePool,
        receipt: &str,
    ) -> Result<Option<Order>, ServerError> {
        let order_opt = sqlx::query_as!(
            Order,
            "SELECT id as \"id: u32\", timestamp, user_email, receipt as \"receipt: Receipt\", payment_intent_id, served as \"served!: bool\" from Orders WHERE receipt = ? AND (expires > CURRENT_TIMESTAMP OR expires IS NULL)",
            receipt
        )
        .fetch_optional(pool)
        .await?;
        Ok(order_opt)
    }

    pub async fn set_email(&mut self, pool: &SqlitePool, email: &str) -> Result<(), ServerError> {
        sqlx::query!(
            "UPDATE Orders SET user_email = ? WHERE id = ?",
            email,
            self.id
        )
        .execute(pool)
        .await?;
        self.user_email = Some(email.to_owned());
        stripe::api::push_metadata(&self.payment_intent_id, "email", email).await?;
        Ok(())
    }

    pub async fn set_served(&mut self, pool: &SqlitePool, served: bool) -> Result<(), ServerError> {
        println!("set_served {} {}", self.id, served);
        sqlx::query!("UPDATE Orders SET served = ? WHERE id = ?", served, self.id)
            .execute(pool)
            .await?;
        self.served = served;
        stripe::api::push_metadata(
            &self.payment_intent_id,
            "commande_servie",
            &served.to_string(),
        )
        .await?;
        Ok(())
    }

    pub async fn get_details(
        &self,
        pool: &SqlitePool,
    ) -> Result<Vec<OrderDetailElement>, ServerError> {
        let detail = sqlx::query!(
            "SELECT
                item_name,
                quantity as \"quantity: u32\",
                tva as \"tva: f32\",
                unit_price_ht as \"unit_price_ht: i32\"
            FROM OrderDetails
            WHERE order_id = ?
                AND quantity != 0",
            self.id
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|e| OrderDetailElement {
            item_name: e.item_name,
            quantity: e.quantity,
            tva: e.tva,
            subtotal_ht: e.unit_price_ht * e.quantity as i32,
            subtotal_ttc: e.unit_price_ht * e.quantity as i32 * (1.0 + e.tva).round() as i32,
        })
        .collect();

        Ok(detail)
    }

    async fn mark_as_paid(
        &mut self,
        pool: &SqlitePool,
        mail_manager: Arc<Box<dyn MailManager>>,
    ) -> Result<(), ServerError> {
        let receipt = Uuid::new_v4().to_string();
        sqlx::query!(
            "UPDATE Orders SET receipt = ?, expires = NULL WHERE id = ?",
            receipt,
            self.id
        )
        .execute(pool)
        .await?;
        stripe::api::push_metadata(&self.payment_intent_id, "reçu", &receipt).await?;
        self.receipt = Some(Receipt(receipt));

        let self_thread = self.clone();
        let pool_thread = pool.to_owned();
        tokio::spawn(async move {
            if let Err(e) = mail::send_qr(&pool_thread, mail_manager, &self_thread).await {
                eprintln!("error while sending receipt mail : {e:?}")
            }
        });
        type ProductId = u32;
        type Volume = f32;
        let detail: Vec<(ProductId, Volume)> = sqlx::query!(
            "SELECT product_id as \"product_id: u32\", quantity, variation_volume as \"variation_volume: f32\"
            FROM OrderDetails
            WHERE order_id = ?",
            self.id
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|r| (r.product_id, r.quantity as f32 * r.variation_volume))
        .collect();
        let mut pool_transaction: Transaction<'static, Sqlite> =
            pool.begin().await.map_err(ServerError::Sqlx)?;

        for (product_id, volume) in detail {
            sqlx::query!(
                "UPDATE Products SET stock_quantity = stock_quantity - ? WHERE id = ?",
                volume,
                product_id
            )
            .execute(&mut *pool_transaction)
            .await
            .map_err(ServerError::Sqlx)?;
        }

        pool_transaction.commit().await.map_err(ServerError::Sqlx)?;
        Ok(())
    }

    pub async fn generate_from_cart(
        pool: &SqlitePool,
        cart: Cart,
    ) -> Result<OrderId, OrderProcessError> {
        let products = products::get_all(pool).await?;
        let variations = Variation::get_all(pool).await?;
        let mut total_price: i32 = 0;
        for cart_element in &cart.elements {
            let variation = variations
                .iter()
                .find(|e| e.id == cart_element.variation_id)
                .ok_or(OrderProcessError::VariationNotFound(
                    cart_element.variation_id,
                ))?;
            let product = products
                .iter()
                .find(|e| e.id == variation.product_id)
                .ok_or(OrderProcessError::ProductNotFound(variation.product_id))?;

            if product.stock_quantity as f32 >= cart_element.quantity as f32 * variation.volume {
                total_price += (variation.price_ht as f32 * (1f32 + variation.tva)) as i32
                    * cart_element.quantity as i32;
            } else {
                return Err(OrderProcessError::NotEnoughStock(
                    product.name.clone(),
                    product.id,
                ));
            }
        }

        let payment_intent = stripe::api::create_payment_intent(total_price as i64).await?;
        let expires = OffsetDateTime::now_utc() + ORDER_DURATION;
        let order_id = sqlx::query!(
            "INSERT INTO Orders (expires, payment_intent_id, client_secret) VALUES (?, ?, ?)",
            expires,
            payment_intent.id,
            payment_intent.client_secret
        )
        .execute(pool)
        .await
        .map_err(ServerError::Sqlx)?
        .last_insert_rowid() as u32;
        stripe::api::push_metadata(&payment_intent.id, "order_id", &order_id.to_string()).await?;
        for cart_element in cart.elements.iter().filter(|e| e.quantity > 0) {
            let variation = variations
                .iter()
                .find(|e| e.id == cart_element.variation_id)
                .ok_or(OrderProcessError::VariationNotFound(
                    cart_element.variation_id,
                ))?;
            let product = products
                .iter()
                .find(|e| e.id == variation.product_id)
                .ok_or(OrderProcessError::ProductNotFound(variation.product_id))?;
            let item_name = if variation.name.is_empty() {
                product.name.clone()
            } else {
                format!("{} ({})", product.name, variation.name)
            };
            sqlx::query!(
                "INSERT INTO OrderDetails(
                    order_id,
                    product_id,
                    item_name,
                    unit_price_ht,
                    tva,
                    quantity,
                    variation_volume
                    ) VALUES (?, ?, ?, ?, ?, ?, ?)",
                order_id,
                variation.product_id,
                item_name,
                variation.price_ht,
                variation.tva,
                cart_element.quantity,
                variation.volume
            )
            .execute(pool)
            .await
            .map_err(ServerError::Sqlx)?;
        }
        let pool = pool.to_owned();
        tokio::spawn(async move {
            let order = Order::get(&pool, order_id)
                .await
                .expect("could not fetch order while setting details metadatas");
            if let Some(order) = order {
                let details = order
                    .get_details(&pool)
                    .await
                    .expect("could not fetch details while setting details metadatas");
                for detail in details {
                    stripe::api::push_metadata(
                        &order.payment_intent_id,
                        &format!("produit: {}", detail.item_name),
                        &format!("quantité : {}", detail.quantity),
                    )
                    .await
                    .expect("could not set metadata");
                }
            }
        });
        Ok(order_id)
    }
    pub async fn get_payment_intent(
        &mut self,
        pool: &SqlitePool,
        mail_manager: Arc<Box<dyn MailManager>>,
    ) -> Result<PaymentIntent, ServerError> {
        let intent = stripe::api::fetch_payment_intent(&self.payment_intent_id).await?;
        if intent.status == PaymentIntentStatus::Succeeded {
            let receipt = sqlx::query!("SELECT receipt FROM Orders WHERE id = ?", self.id)
                .fetch_one(pool)
                .await?
                .receipt;
            if receipt.is_none() {
                self.mark_as_paid(pool, mail_manager).await?;
            }
        }
        Ok(intent)
    }
    pub async fn get_full_price_ht(&self, pool: &SqlitePool) -> Result<i32, ServerError> {
        let total = sqlx::query!(
            "SELECT cast(SUM(unit_price_ht * quantity) as int) as result
            FROM OrderDetails WHERE order_id = ?;",
            self.id
        )
        .fetch_one(pool)
        .await
        .map_err(ServerError::Sqlx)?;

        let res = total
            .result
            .ok_or(ServerError::Sqlx(sqlx::Error::RowNotFound))?;
        Ok(res as i32)
    }
    pub async fn get_full_price_ttc(&self, pool: &SqlitePool) -> Result<i32, ServerError> {
        let total = sqlx::query!(
            "SELECT cast(SUM(unit_price_ht * (1 + tva) * quantity) as int) as result
            FROM OrderDetails WHERE order_id = ?;",
            self.id
        )
        .fetch_one(pool)
        .await
        .map_err(ServerError::Sqlx)?;

        let res = total
            .result
            .ok_or(ServerError::Sqlx(sqlx::Error::RowNotFound))?;
        Ok(res as i32)
    }
}

pub async fn search_orders(
    pool: &SqlitePool,
    email: Option<&str>,
    date_begin: Option<OffsetDateTime>,
    date_end: Option<OffsetDateTime>,
    receipt: Option<&str>,
) -> Result<Vec<Order>, ServerError> {
    let email = email.unwrap_or("");
    let receipt = receipt.unwrap_or("");
    let date_begin = date_begin.unwrap_or(OffsetDateTime::UNIX_EPOCH);
    let orders = if let Some(date_end) = date_end {
        sqlx::query_as!(
            Order,
            "SELECT id as \"id: u32\", timestamp, user_email, receipt as \"receipt: Receipt\", payment_intent_id, served as \"served!: bool\"  from Orders
            WHERE receipt IS NOT NULL AND user_email LIKE CONCAT('%', ?, '%') AND receipt LIKE CONCAT('%', ?, '%') AND timestamp > ? AND timestamp < ? ORDER BY timestamp DESC",
            email,
            receipt,
            date_begin,
            date_end
        ).fetch_all(pool).await?
    } else {
        sqlx::query_as!(
            Order,
            "SELECT id as \"id: u32\", timestamp, user_email, receipt as \"receipt: Receipt\", payment_intent_id, served as \"served!: bool\" from Orders
            WHERE user_email LIKE CONCAT('%', ?, '%') AND receipt LIKE CONCAT('%', ?, '%') AND timestamp > ? ORDER BY timestamp DESC",
            email,
            receipt,
            date_begin,
        ).fetch_all(pool).await?
    };
    Ok(orders)
}

pub fn cancel_expired_orders(pool: &SqlitePool) {
    let pool = pool.to_owned();
    tokio::spawn(async move {
        let expired_payment_intents =
            sqlx::query!("SELECT payment_intent_id from Orders WHERE expires < CURRENT_TIMESTAMP AND canceled = FALSE AND receipt IS NULL")
                .fetch_all(&pool)
                .await
                .unwrap();

        for payment_intent in expired_payment_intents {
            stripe::api::mark_as_canceled(&payment_intent.payment_intent_id)
                .await
                .unwrap();
            sqlx::query!(
                "UPDATE Orders SET canceled = TRUE WHERE payment_intent_id = ?",
                payment_intent.payment_intent_id
            )
            .execute(&pool)
            .await
            .unwrap();
        }
    });
}
