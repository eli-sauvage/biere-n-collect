use std::env;

use rocket::{
    data::{self, FromData, ToByteUnit},
    http::Status,
    request::{self, FromRequest},
    response::status::BadRequest,
    serde::json::{json, Json, Value},
    Data, Request, State,
};
use sqlx::{MySql, Pool};
use stripe::{
    CheckoutSession, Client, CreatePaymentIntent, CreatePaymentIntentAutomaticPaymentMethods,
    Currency, EventObject, EventType, PaymentIntent, Webhook,
};

use super::orders::Order;

pub struct PaymentManager {
    pub stripe_client: Client,
    pub publishable_key: String,
}

impl PaymentManager {
    pub fn new() -> PaymentManager {
        let secret_key =
            env::var("STRIPE_SECRET_KEY").expect("stripe secret key is missing from env");
        let publishable_key =
            env::var("STRIPE_PUBLISHABLE_KEY").expect("stripe publishable key is missing from env");
        PaymentManager {
            publishable_key,
            stripe_client: Client::new(secret_key),
        }
    }
}

#[get("/config")]
pub fn get_config(payment_manager: &State<PaymentManager>) -> Json<Value> {
    Json(json!({
        "publishableKey": payment_manager.publishable_key.clone(),
    }))
}

// pub struct Intent
#[get("/create-payment-intent?<order_id>")]
pub async fn create_payment_intent(
    pool: &State<Pool<MySql>>,
    order_id: u32,
    payment_manager: &State<PaymentManager>,
) -> Result<Json<Value>, BadRequest<String>> {
    let order = match Order::from_id(pool, order_id).await.unwrap() {
        Some(order) => order,
        None => return Err(BadRequest("the order was not found".to_string())),
    };
    let price = match order.get_full_price(pool).await {
        Ok(price) => price,
        Err(e) => {
            eprintln!("could not get full price of order {} : {e:?}", order_id);
            return Err(BadRequest("internal db error".to_string()));
        }
    };
    let intent = {
        let mut create_intent = CreatePaymentIntent::new(price, Currency::EUR);
        let automatic_payment_method = CreatePaymentIntentAutomaticPaymentMethods {
            enabled: true,
            allow_redirects: Some(
                stripe::CreatePaymentIntentAutomaticPaymentMethodsAllowRedirects::Always,
            ),
        };
        create_intent.automatic_payment_methods = Some(automatic_payment_method);
        // create_intent.return_url = Some("http://localhost:5173/return");
        // create_intent.confirm = Some(true);
        // create_intent.payment_method_types = Some(vec!["".to_string()]);
        // create_intent.statement_descriptor = Some("test descriptor");
        create_intent.metadata = Some(
            [("color".to_string(), "red".to_string())]
                .iter()
                .cloned()
                .collect(),
        );
        PaymentIntent::create(&payment_manager.stripe_client, create_intent)
            .await
            .map_err(|e| {
                eprintln!("error generating payment intent : {e:?}");
                BadRequest("could not generate payment intent".to_string())
            })?
    };
    let client_secret = intent
        .client_secret
        .ok_or(BadRequest("could not generate client_secret".to_string()))?;

    Ok(Json(
        json!({"clientSecret": client_secret, "total_price": price}),
    ))
}

#[post("/stripe_webhooks", data = "<payload>")]
pub async fn stripe_webhooks(stripe_signature: StripeSignature<'_>, payload: Payload) -> Status {
    if let Ok(event) = Webhook::construct_event(
        &payload.contents,
        stripe_signature.signature,
        "webhook_secret_key",
    ) {
        match event.type_ {
            EventType::CheckoutSessionCompleted => {
                if let EventObject::CheckoutSession(session) = event.data.object {
                    match checkout_session_completed(session) {
                        Ok(_) => Status::Accepted,
                        Err(_) => Status::BadRequest,
                    }
                } else {
                    Status::BadRequest
                }
            }
            _ => Status::Accepted,
        }
    } else {
        Status::BadRequest
    }
}

fn checkout_session_completed<'a>(session: CheckoutSession) -> Result<(), &'a str> {
    println!("Checkout Session Completed");
    println!("{:?}", session.id);
    Ok(())
}

pub struct Payload {
    pub contents: String,
}

#[derive(Debug)]
pub enum FromDataError {
    TooLarge,
    NoColon,
    InvalidAge,
    Io(std::io::Error),
}

#[rocket::async_trait]
impl<'r> FromData<'r> for Payload {
    type Error = FromDataError;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use rocket::outcome::Outcome::*;
        use FromDataError::*;

        let limit = req
            .limits()
            .get("form")
            .unwrap_or_else(|| 1_000_000.bytes());

        let contents = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Error((Status::PayloadTooLarge, TooLarge)),
            Err(e) => return Error((Status::InternalServerError, Io(e))),
        };
        Success(Payload { contents })
    }
}

pub struct StripeSignature<'a> {
    pub signature: &'a str,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for StripeSignature<'r> {
    type Error = &'r str;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.headers().get_one("Stripe-Signature") {
            None => request::Outcome::Error((Status::BadRequest, "No signature provided")),
            Some(signature) => request::Outcome::Success(StripeSignature { signature }),
        }
    }
}
