use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;

use crate::app::orders::OrderId;

use super::ServerError;

#[derive(Error, Debug)]
pub enum PaymentIntentError {
    #[error("le bar est fermé! impossible de continuer")]
    BarIsClosed,
    #[error("la commande <id={0}> n'a pas été trouvée")]
    OrderNotFound(OrderId),
    #[error("La commande n'a pas été trouvée à partir du token")]
    OrderNotFoundFromSecrets,
    #[error("Le reçu n'a pas encore été crée, merci de réessayer après avoir payer")]
    NoReceipt,
    #[error("Cette commande a déjà été payée")]
    AlreadyPaid,
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
                Self::BarIsClosed => StatusCode::SERVICE_UNAVAILABLE,
                Self::AlreadyPaid => StatusCode::BAD_REQUEST,
                Self::ServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (status, Json(json!({"error": self.to_string()}))).into_response()
        }
    }
}
