use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

use super::{ErrorResponse, ServerError};

#[derive(Error, Debug)]
pub enum ManageStockError {
    #[error("le stock avec l'id {0} n'existe pas")]
    StockNotFound(u32),
    #[error("stock with id {0} cannot move up")]
    CannotMoveUp(u32),
    #[error("stock with id {0} cannot move down")]
    CannotMoveDown(u32),
    #[error("server error")]
    ServerError(#[from] ServerError),
}
impl IntoResponse for ManageStockError {
    fn into_response(self) -> axum::response::Response {
        if let ManageStockError::ServerError(e) = self {
            e.into_response()
        } else {
            let status = match self {
                Self::StockNotFound(_) => StatusCode::NOT_FOUND,
                Self::CannotMoveUp(_) | Self::CannotMoveDown(_) => StatusCode::BAD_REQUEST,
                Self::ServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (status, ErrorResponse::json(self.to_string())).into_response()
        }
    }
}
