use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

use super::{ErrorResponse, ServerError};

#[derive(Error, Debug)]
pub enum OrderProcessError {
    #[error("le bar est fermé! impossible de continuer")]
    BarIsClosed,
    #[error("pas assez de stock pour l'item {0}<#{1}>")]
    NotEnoughStock(String, u32),
    #[error("product not found (id = {0})")]
    ProductNotFound(u32),
    #[error("variation not found (id = {0})")]
    VariationNotFound(u32),
    #[error("la commande est vide")]
    EmptyOrder,
    #[error("server error")]
    ServerError(#[from] ServerError),
}
impl IntoResponse for OrderProcessError {
    fn into_response(self) -> axum::response::Response {
        if let OrderProcessError::ServerError(e) = self {
            e.into_response()
        } else {
            let status = match self {
                Self::NotEnoughStock(_, _)
                | Self::ProductNotFound(_)
                | Self::VariationNotFound(_)
                | Self::EmptyOrder => StatusCode::BAD_REQUEST,
                Self::BarIsClosed => StatusCode::SERVICE_UNAVAILABLE,
                Self::ServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };

            (status, ErrorResponse::json(self.to_string())).into_response()
        }
    }
}

#[derive(Error, Debug)]
pub enum OrderManagementError {
    #[error("invalid date provided")]
    InvalidDate,
    #[error("order not found")]
    OrderNotFound,
    #[error("server error")]
    ServerError(#[from] ServerError),
}
impl IntoResponse for OrderManagementError {
    fn into_response(self) -> axum::response::Response {
        if let Self::ServerError(e) = self {
            e.into_response()
        } else {
            let status = match self {
                Self::InvalidDate | Self::OrderNotFound => StatusCode::BAD_REQUEST,
                Self::ServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (status, ErrorResponse::json(self.to_string())).into_response()
        }
    }
}

#[derive(Error, Debug)]
pub enum SendReceiptEmailError {
    #[error("could not find user email address")]
    NoEmailAddress,
    #[error("invalid email address")]
    InvalidEmailAddress(#[from] lettre::address::AddressError),
    #[error("no receipt found in current order")]
    NoReceipt,
    #[error("error while generating qr code image")]
    ImageError(#[from] image::error::ImageError),
    #[error("server error")]
    ServerError(#[from] ServerError),
}
