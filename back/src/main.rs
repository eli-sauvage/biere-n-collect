mod db;
mod errors;
mod models;
#[macro_use]
extern crate rocket;

use errors::Error;
use models::payments::{create_payment_intent, get_config, stripe_webhooks, PaymentManager};
use models::stock_manager::{
    self as stocks_module, CartValidationResponse, IncomingOrder, Stock, StockManager,
};
use rocket::response::status::{BadRequest, NotFound};
use rocket::{serde::json::Json, State};
use rocket_cors::{AllowedOrigins, CorsOptions};
use sqlx::{MySql, Pool};

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[get("/stocks")]
async fn stocks(pool: &State<Pool<MySql>>) -> Result<Json<Vec<Stock>>, NotFound<String>> {
    let p = stocks_module::get_all_stocks(pool).await.map_err(|e| {
        eprintln!("error retreiving stocks frmo db : {e:?}");
        NotFound("error retreiving data from db".to_string())
    })?;
    Ok(Json(p))
}

#[post("/validate_cart", data = "<cart>")]
async fn validate_cart(
    pool: &State<Pool<MySql>>,
    stock_manager: &State<StockManager>,
    cart: Json<IncomingOrder>,
) -> Result<Json<CartValidationResponse>, BadRequest<String>> {
    match stock_manager.process_order(pool, cart.clone().0).await {
        Ok(order_id) => return Ok(Json(CartValidationResponse { order_id: order_id })),
        Err(Error::Order(e)) => return Err(BadRequest(e.to_string())),
        Err(e) => {
            panic!("error validating cart {cart:?} : {e:?}");
        }
    }
}

#[rocket::main]
async fn main() -> Result<(), Error> {
    let pool = db::setup_db_and_migrate().await;
    let stock_manager = StockManager::new();
    let payment_manager = PaymentManager::new();

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allow_credentials(true);

    let _rocket = rocket::build()
        .manage(pool)
        .manage(stock_manager)
        .manage(payment_manager)
        .attach(cors.to_cors().unwrap())
        .mount("/hello", routes![world])
        .mount("/api", routes![stocks, validate_cart, get_config, create_payment_intent, stripe_webhooks])
        .launch()
        .await
        .map_err(Error::Rocket)?;

    Ok(())
}
