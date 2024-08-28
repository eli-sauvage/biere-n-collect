use axum::response::IntoResponse;
use reqwest::StatusCode;
use thiserror::Error;

use super::{ErrorResponse, ServerError};

#[derive(Error, Debug)]
pub enum OrderProcessError {
    #[error("pas assez de stock pour l'item {0}<#{1}>")]
    NotEnoughStock(String, u32),
    #[error("prouct not found (id = {0})")]
    ProductNotFound(u32),
    #[error("server error")]
    ServerError(#[from] ServerError),
}
impl IntoResponse for OrderProcessError {
    fn into_response(self) -> axum::response::Response {
        if let OrderProcessError::ServerError(e) = self {
            e.into_response()
        } else {
            let status = match self {
                Self::NotEnoughStock(_, _) | Self::ProductNotFound(_) => StatusCode::BAD_REQUEST,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };

            (status, ErrorResponse::json(self.to_string())).into_response()
        }
    }
}

#[derive(Error, Debug)]
pub enum OrderManagementError {
    // #[error("User could not be identified: {0}")]
    // NotAuthorized(ParseUserErrorMsg),
    #[error("server error")]
    ServerError(#[from] ServerError),
}
impl IntoResponse for OrderManagementError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::ServerError(e) => e.into_response(),
        }
        // if let OrderManagementError::ServerError(e) = self{
        //     e.into_response()
        // }else{
        //     todo!()
        // }
    }
}
