use std::{collections::HashMap, env};

use crate::{app::stripe::payment_intents::PaymentIntent, errors::ServerError};
use reqwest::Client;

use super::payment_intents::PaymentIntentId;

pub type SecretKey = String;
fn get_secret_key() -> Result<SecretKey, ServerError> {
    let key = env::var("STRIPE_SECRET_KEY")
        .map_err(|e| ServerError::MissingEnv("STRIPE_SECRET_KEY".into(), e))?;
    Ok(key)
}

pub async fn create_payment_intent(amount: i64) -> Result<PaymentIntent, ServerError> {
    let url = "https://api.stripe.com/v1/payment_intents";
    let client = Client::new();

    let amount = amount.to_string();
    let mut params = HashMap::new();
    params.insert("amount", amount.as_str()); // Amount in the smallest currency unit (e.g., cents for USD)
    params.insert("currency", "eur"); // Currency code
    params.insert("automatic_payment_methods[enabled]", "true"); // Payment method types

    let response = client
        .post(url)
        .basic_auth(get_secret_key()?, Some("")) // Basic auth with the secret key
        .form(&params) // Send the parameters as a form
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        // Deserialize the response into the PaymentIntent struct
        let payment_intent: PaymentIntent = response.json().await?;
        println!("Payment Intent created: {:?}", payment_intent);
        Ok(payment_intent)
    } else {
        // If the request failed, print the status and body
        let status = response.status();
        let body = response.text().await?;
        Err(ServerError::StripeApi(status, body))
    }
}
pub async fn fetch_payment_intent(
    payment_intent_id: &PaymentIntentId,
) -> Result<PaymentIntent, ServerError> {
    let url = format!(
        "https://api.stripe.com/v1/payment_intents/{}",
        payment_intent_id
    );
    let client = Client::new();

    let response = client
        .get(url)
        .basic_auth(get_secret_key()?, Some("")) // Basic auth with the secret key
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        // Deserialize the response into the PaymentIntent struct
        let payment_intent: PaymentIntent = response.json().await?;
        Ok(payment_intent)
    } else {
        // If the request failed, print the status and body
        let status = response.status();
        let body = response.text().await?;
        Err(ServerError::StripeApi(status, body))
    }
}
// pub async fn fetch_status(
//     &self,
//     pool: &Pool<MySql>,
//     stock_manager: &StockManager,
//     order_id: OrderId,
// ) -> Result<PaymentStatus, PaymentIntentError> {
//     let payment = Payment::get(pool, order_id)
//         .await?
//         .ok_or_else(|| PaymentIntentError::IntentNotCreatedYet(order_id))?;

//     if payment.status != PaymentStatus::Processing {
//         return Ok(payment.status);
//     }
//     let url = format!(
//         "https://api.stripe.com/v1/payment_intents/{}",
//         payment.payment_intent_id
//     );
//     let client = Client::new();

//     let response = client
//         .post(url)
//         .basic_auth(self.secret_key.clone(), Some("")) // Basic auth with the secret key
//         .send()
//         .await
//         .map_err(ServerError::Reqwest)?;

//     // Check if the request was successful
//     if response.status().is_success() {
//         // Deserialize the response into the PaymentIntent struct
//         let payment_intent: PaymentIntent = response.json().await.map_err(ServerError::Reqwest)?;
//         println!("GOT STATUS : {:?}", payment_intent.status);
//         let status = match payment_intent.status.as_str() {
//             "canceled" => PaymentStatus::Canceled,
//             "succeeded" => PaymentStatus::Succeeded,
//             _ => PaymentStatus::Processing,
//         };
//         if status != payment.status {
//             Payment::update_status(pool, payment.payment_intent_id, &status).await?;
//             if status == PaymentStatus::Succeeded {
//                 if let Some(mut order) = Order::get(pool, order_id).await? {
//                     order.mark_as_paid(pool, stock_manager).await?;
//                 }
//             }
//         }
//         Ok(status)
//     } else {
//         // If the request failed, print the status and body
//         let status = response.status();
//         let body = response.text().await.map_err(ServerError::Reqwest)?;
//         println!(
//             "Failed to create Payments Intent. Status: {}. Body: {}",
//             status, body
//         );
//         Err(PaymentIntentError::CouldNotFetchIntent)
//     }
// }
