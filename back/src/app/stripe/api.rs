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

pub async fn push_metadata(
    payment_intent_id: &PaymentIntentId,
    key: &str,
    value: &str,
) -> Result<(), ServerError> {
    let (payment_intent_id, key, value) =
        (payment_intent_id.clone(), key.to_owned(), value.to_owned());
    tokio::spawn(async move {
        let url = format!(
            "https://api.stripe.com/v1/payment_intents/{}?metadata[{}]={}",
            payment_intent_id, key, value
        );
        let client = Client::new();

        client
            .post(url)
            .basic_auth(get_secret_key().unwrap(), Some("")) // Basic auth with the secret key
            .send()
            .await
            .unwrap();
    });
    Ok(())
}

pub async fn mark_as_canceled(payment_intent_id: &PaymentIntentId) -> Result<(), ServerError> {
    let url = format!(
        "https://api.stripe.com/v1/payment_intents/{}/cancel?cancellation_reason=abandoned",
        payment_intent_id
    );
    let client = Client::new();

    let response = client
        .post(url)
        .basic_auth(get_secret_key()?, Some("")) // Basic auth with the secret key
        .send()
        .await?;
    if response.status().is_success() {
        Ok(())
    } else {
        // If the request failed, print the status and body
        let status = response.status();
        let body = response.text().await?;
        Err(ServerError::StripeApi(status, body))
    }
}
