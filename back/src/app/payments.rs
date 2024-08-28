
// use rocket::{
//     serde::json::{json, Json, Value},
//     State,
// };
// use serde::{Deserialize, Serialize};
// use sqlx::{MySql, Pool};

// use crate::errors::{PaymentIntentError, ServerError};

// use super::{
//     orders::{Order, OrderId},
// };

// #[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type, PartialEq)]
// #[serde(rename_all = "lowercase")]
// #[sqlx(rename_all = "lowercase")]
// pub enum PaymentStatus {
//     Canceled,
//     Processing,
//     Succeeded,
// }

// pub struct Payment {
//     payment_intent_id: String,
//     status: PaymentStatus,
// }

// impl Payment {
//     pub async fn insert(
//         pool: &Pool<MySql>,
//         order_id: OrderId,
//         payment_intent_id: &String,
//         status: PaymentStatus,
//     ) -> Result<(), ServerError> {
//         // let status_str = json::to_string(&status)?;
//         sqlx::query!(
//             "INSERT INTO Payments (order_id, payment_intent_id, status) VALUES (?, ?, ?)",
//             order_id,
//             payment_intent_id,
//             status
//         )
//         .execute(pool)
//         .await?;
//         Ok(())
//     }

//     pub async fn get(
//         pool: &Pool<MySql>,
//         order_id: OrderId,
//     ) -> Result<Option<Payment>, ServerError> {
//         let payment = sqlx::query_as!(Payment,
//             "SELECT status as \"status: PaymentStatus\", payment_intent_id FROM Payments WHERE order_id = ?",
//             order_id
//         )
//         .fetch_optional(pool)
//         .await
//         .map_err(ServerError::Sqlx)?;

//         Ok(payment)
//     }

//     pub async fn update_status(
//         pool: &Pool<MySql>,
//         payment_intent_id: String,
//         status: &PaymentStatus,
//     ) -> Result<(), ServerError> {
//         sqlx::query!(
//             "UPDATE Payments SET status = ? WHERE payment_intent_id = ?",
//             status,
//             payment_intent_id
//         )
//         .execute(pool)
//         .await?;
//         Ok(())
//     }
// }



// pub struct PaymentManager {
//     pub secret_key: String,
//     pub publishable_key: String,
// }

// impl PaymentManager {
//     pub fn new() -> PaymentManager {
//         let secret_key =
//             env::var("STRIPE_SECRET_KEY").expect("stripe secret key is missing from env");
//         let publishable_key =
//             env::var("STRIPE_PUBLISHABLE_KEY").expect("stripe publishable key is missing from env");
//         PaymentManager {
//             publishable_key,
//             secret_key,
//         }
//     }

    
// }


// // #[get("/create-payment-intent?<order_id>")]
// pub async fn create_payment_intent(
//     pool: &State<Pool<MySql>>,
//     order_id: OrderId,
//     payment_manager: &State<PaymentManager>,
// ) -> Result<Json<Value>, PaymentIntentError> {
//     let order = Order::get(pool, order_id)
//         .await?
//         .ok_or_else(|| PaymentIntentError::OrderNotFound(order_id))?;
//     let price = order.get_full_price(pool).await?;

//     let intent = payment_manager
//         .create_payment_intent(pool, order.id, price)
//         .await?;

//     order.set_payment_intent_id(pool, intent.id).await?;

//     Ok(Json(
//         json!({"clientSecret": intent.client_secret, "total_price": price}),
//     ))
// }

// // #[get("/status?<order_id>")]
// pub async fn get_payment_status(
//     pool: &State<Pool<MySql>>,
//     payment_manager: &State<PaymentManager>,
//     stock_manager: &State<StockManager>,
//     order_id: OrderId,
// ) -> Result<Json<Value>, PaymentIntentError> {
//     let status = &payment_manager
//         .fetch_status(pool, stock_manager, order_id)
//         .await?;

//     Ok(Json(json!({"status": status})))
// }
