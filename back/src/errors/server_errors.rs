use std::env::VarError;

use axum::{http::StatusCode, response::IntoResponse};

use thiserror::Error;

use super::ErrorResponse;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("sqlx error")]
    Sqlx(#[from] sqlx::error::Error),
    #[error("Migration error")]
    SqlxMigrate(#[from] sqlx::migrate::MigrateError),
    #[error("uuid error")]
    Uuid(#[from] uuid::Error),
    #[error("reqwest error")]
    Reqwest(#[from] reqwest::Error),
    #[error("serde json error")]
    SerdeJson(#[from] serde_json::error::Error),
    #[error("missing env : {0}. Error: {1}")]
    MissingEnv(String, VarError),
    #[error("stripe api error : status = {0}, body =  {1}")]
    StripeApi(StatusCode, String),
    #[error("email address error")]
    EmailAddress(#[from] lettre::address::AddressError),
    #[error("email build error")]
    EmailBuild(#[from] lettre::error::Error),
    #[error("email send error")]
    EmailSend(#[from] lettre::transport::smtp::Error),
    #[error("could not generate qr code")]
    QrCode(#[from] qrcode::types::QrError),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        eprintln!("----");
        eprintln!("Internal server error : {self:?}");
        eprintln!("{self}");
        eprintln!("----");

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse::json("internal server error".to_string()),
        )
            .into_response()
    }
}
