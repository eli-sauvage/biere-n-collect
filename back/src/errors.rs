use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("sqlx error")]
    Sqlx(#[from] sqlx::error::Error),
    #[error("Migration error")]
    SqlxMigrate(#[from] sqlx::migrate::MigrateError),
    #[error("rocket error")]
    Rocket(#[from] rocket::Error),
    #[error("order error : {0}")]
    Order(OrderError)
}

#[derive(Error, Debug)]
pub enum OrderError{
    #[error("pas assez de stock pour l'item {0}<#{1}>")]
    NotEnoughStock(String, u32),
    #[error("prouct not found in db (id = {0})")]
    ProductNotFound(u32)
}