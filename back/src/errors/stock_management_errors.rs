use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

use super::{ErrorResponse, ServerError};

#[derive(Error, Debug)]
pub enum ManageStockError {
    #[error("la variation de produit avec l'id {0} n'existe pas")]
    VariationNotFound(u32),
    #[error("le produit avec l'id {0} n'existe pas")]
    ProductNotFound(u32),
    #[error("server error")]
    ServerError(#[from] ServerError),
}
impl IntoResponse for ManageStockError {
    fn into_response(self) -> axum::response::Response {
        if let ManageStockError::ServerError(e) = self {
            e.into_response()
        } else {
            let status = match self {
                Self::ProductNotFound(_)
                | Self::VariationNotFound(_) => StatusCode::NOT_FOUND,
                Self::ServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (status, ErrorResponse::json(self.to_string())).into_response()
        }
    }
}
