use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;

use crate::app::orders::OrderId;

use super::ServerError;

#[derive(Error, Debug)]
pub enum PaymentIntentError {
    #[error("order with id {0} not found")]
    OrderNotFound(OrderId),
    #[error("order not found from the given secrets")]
    OrderNotFoundFromSecrets,
    #[error("the receipt does not exist yet, please try again after paying")]
    NoReceipt,
    #[error("server error")]
    ServerError(#[from] ServerError),
}
impl IntoResponse for PaymentIntentError {
    fn into_response(self) -> axum::response::Response {
        if let PaymentIntentError::ServerError(e) = self {
            e.into_response()
        } else {
            let status = match self {
                Self::OrderNotFound(_) | Self::OrderNotFoundFromSecrets | Self::NoReceipt => {
                    StatusCode::NOT_FOUND
                }
                Self::ServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (status, Json(json!({"error": self.to_string()}))).into_response()
        }
    }
}
