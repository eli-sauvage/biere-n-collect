use std::{collections::HashMap, env};

use reqwest::Client;
use rocket::{
    data::{self, FromData, ToByteUnit},
    http::Status,
    request::{self, FromRequest},
    response::status::BadRequest,
    serde::json::{json, Json, Value},
    Data, Request, State,
};
use serde::Deserialize;
use sqlx::{MySql, Pool};
// use stripe::{
//     CheckoutSession, Client, CreatePaymentIntent, CreatePaymentIntentAutomaticPaymentMethods,
//     Currency, EventObject, EventType, PaymentIntent, Webhook,
// };
// use stripe::Client;
// use stripe_checkout::CheckoutSession;
// use stripe_core::payment_intent::{
//     CreatePaymentIntent, CreatePaymentIntentAutomaticPaymentMethods,
//     CreatePaymentIntentAutomaticPaymentMethodsAllowRedirects,
// };
// use stripe_types::Currency;
// use stripe_webhook::{EventObject, EventType, Webhook};

use crate::errors::{PaymentIntentError, ServerError};

use super::orders::Order;

#[derive(Deserialize, Debug)]
struct PaymentIntent {
    id: String,
    client_secret: String,
    // Include other fields you are interested in
}

pub struct PaymentManager {
    pub secret_key: String,
    pub publishable_key: String,
    pub webhook_secret_key: String,
}

impl PaymentManager {
    pub fn new() -> PaymentManager {
        let secret_key =
            env::var("STRIPE_SECRET_KEY").expect("stripe secret key is missing from env");
        let publishable_key =
            env::var("STRIPE_PUBLISHABLE_KEY").expect("stripe publishable key is missing from env");
        let webhook_secret_key =
            env::var("STRIPE_WEBHOOK_SECRET_KEY").expect("stripe secret key is missing from env");
        PaymentManager {
            publishable_key,
            secret_key,
            webhook_secret_key,
        }
    }

    pub async fn create_payment_intent(
        &self,
        amount: i64,
    ) -> Result<PaymentIntent, PaymentIntentError> {
        let url = "https://api.stripe.com/v1/payment_intents";
        let client = Client::new();

        let amount = amount.to_string();
        let mut params = HashMap::new();
        params.insert("amount", amount.as_str()); // Amount in the smallest currency unit (e.g., cents for USD)
        params.insert("currency", "eur"); // Currency code
        params.insert("automatic_payment_method[enabled]", "true"); // Payment method types

        let response = client
            .post(url)
            .basic_auth(self.secret_key.clone(), Some("")) // Basic auth with the secret key
            .form(&params) // Send the parameters as a form
            .send()
            .await
            .map_err(ServerError::Reqwest)?;

        // Check if the request was successful
        if response.status().is_success() {
            // Deserialize the response into the PaymentIntent struct
            let payment_intent: PaymentIntent =
                response.json().await.map_err(ServerError::Reqwest)?;
            println!("Payment Intent created: {:?}", payment_intent);
            Ok(payment_intent)
        } else {
            // If the request failed, print the status and body
            let status = response.status();
            let body = response.text().await.map_err(ServerError::Reqwest)?;
            println!(
                "Failed to create Payment Intent. Status: {}. Body: {}",
                status, body
            );
            Err(PaymentIntentError::CouldNotCreateIntent)
        }
    }
}

#[get("/config")]
pub fn get_config(payment_manager: &State<PaymentManager>) -> Json<Value> {
    Json(json!({
        "publishableKey": payment_manager.publishable_key.clone(),
    }))
}

#[get("/create-payment-intent?<order_id>")]
pub async fn create_payment_intent(
    pool: &State<Pool<MySql>>,
    order_id: u32,
    payment_manager: &State<PaymentManager>,
) -> Result<Json<Value>, PaymentIntentError> {
    let mut order = Order::from_id(pool, order_id)
        .await?
        .ok_or_else(|| PaymentIntentError::OrderNotFound(order_id))?;
    let price = order.get_full_price(pool).await?;

    let intent = payment_manager.create_payment_intent(price).await?;

    order.set_payment_intent_id(pool, intent.id).await?;

    Ok(Json(
        json!({"clientSecret": intent.client_secret, "total_price": price}),
    ))
}
// let intent = {
//     let automatic_payment_method = CreatePaymentIntentAutomaticPaymentMethods {
//         enabled: true,
//         allow_redirects: Some(CreatePaymentIntentAutomaticPaymentMethodsAllowRedirects::Always),
//     };
//     let create_intent = CreatePaymentIntent::new(price, Currency::EUR)
//         .automatic_payment_methods(automatic_payment_method);
//     create_intent.send(&payment_manager.stripe_client).await?
// };
// let client_secret = intent
//     .client_secret
//     .ok_or(PaymentIntentError::NoClientSecretInIntent)?;

// order.set_payment_intent_id(pool, intent.id).await?;

// Ok(Json(
//     json!({"clientSecret": client_secret, "total_price": price}),
// ))
// }

#[post("/stripe_webhooks", data = "<payload>")]
pub async fn stripe_webhooks(payload: Json<Value>) -> Status {
    Status::Accepted
}
// if let Ok(event) = Webhook::construct_event(
//     &payload.contents,
//     stripe_signature.signature,
//     &env::var("STRIPE_WEBHOOK_SECRET_KEY").unwrap(),
// ) {
//     println!("{:?}", event.type_);
//     match event.type_ {
//         EventType::PaymentIntentSucceeded => {
//             if let EventObject::PaymentIntentSucceeded(intent) = event.data.object {
//                 println!("CHECKOUT COMPLETE");
//                 Status::Accepted
//             } else {
//                 Status::BadRequest
//             }
//         }
//         _ => Status::Accepted,
//     }
// } else {
//     Status::BadRequest
// }
// }

// fn checkout_session_completed<'a>(session: CheckoutSession) -> Result<(), &'a str> {
//     println!("Checkout Session Completed");
//     println!("{:?}", session.id);
//     Ok(())
// }

// pub struct Payload {
//     pub contents: String,
// }

// #[derive(Debug)]
// pub enum FromDataError {
//     TooLarge,
//     Io,
// }

// #[rocket::async_trait]
// impl<'r> FromData<'r> for Payload {
//     type Error = FromDataError;

//     async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
//         use rocket::outcome::Outcome::*;
//         use FromDataError::*;

//         let limit = req
//             .limits()
//             .get("form")
//             .unwrap_or_else(|| 1_000_000.bytes());

//         let contents = match data.open(limit).into_string().await {
//             Ok(string) if string.is_complete() => string.into_inner(),
//             Ok(_) => return Error((Status::PayloadTooLarge, TooLarge)),
//             Err(e) => {
//                 eprintln!("server error on request : {e:?}");
//                 return Error((Status::InternalServerError, Io));
//             }
//         };
//         Success(Payload { contents })
//     }
// }

// pub struct StripeSignature<'a> {
//     pub signature: &'a str,
// }

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for StripeSignature<'r> {
//     type Error = &'r str;

//     async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
//         match req.headers().get_one("Stripe-Signature") {
//             None => request::Outcome::Error((Status::BadRequest, "No signature provided")),
//             Some(signature) => request::Outcome::Success(StripeSignature { signature }),
//         }
//     }
// }
