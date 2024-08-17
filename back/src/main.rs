mod db;
mod errors;
mod models;
mod users;
#[macro_use]
extern crate rocket;

use errors::ServerError;
use models::stock_manager::{get_stocks, update_stock, StockManager};
use models::{
    orders::validate_cart,
    payments::{create_payment_intent, get_config, stripe_webhooks, PaymentManager},
};
use rocket_cors::{AllowedOrigins, CorsOptions};
use users::{
    challenge::{create_challenge, verify_challenge, ChallengeManager},
    mail::MailManager,
};

#[rocket::main]
async fn main() -> Result<(), ServerError> {
    dotenvy::dotenv().expect("could not load env from .env file");
    let pool = db::setup_db_and_migrate().await;
    let stock_manager = StockManager::new();
    let payment_manager = PaymentManager::new();
    let mail_manager = MailManager::new();
    let challenge_manager = ChallengeManager::new();

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allow_credentials(true);

    let _rocket = rocket::build()
        .manage(pool)
        .manage(stock_manager)
        .manage(payment_manager)
        .manage(mail_manager)
        .manage(challenge_manager)
        .attach(cors.to_cors().unwrap())
        .mount(
            "/api",
            routes![
                validate_cart,
                get_config,
                create_payment_intent,
                stripe_webhooks
            ],
        )
        .mount("/api/order", routes![validate_cart])
        .mount("/api/stock", routes![get_stocks, update_stock])
        .mount(
            "/api/challenge",
            routes![create_challenge, verify_challenge],
        )
        .launch()
        .await
        .map_err(ServerError::Rocket)?;

    Ok(())
}
