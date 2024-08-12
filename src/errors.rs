use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("sqlx error")]
    Sqlx(#[from] sqlx::error::Error),
    #[error("Migration error")]
    SqlxMigrate(#[from] sqlx::migrate::MigrateError),
}