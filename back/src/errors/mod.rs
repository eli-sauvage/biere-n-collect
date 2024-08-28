mod server_errors;
use axum::Json;
use serde::Serialize;
pub use server_errors::ServerError;

mod admin_errors;
pub use admin_errors::SessionError;
pub use admin_errors::UserManagementError;
pub use admin_errors::UserParseError;

mod order_errors;
pub use order_errors::OrderManagementError;
pub use order_errors::OrderProcessError;

mod stock_management_errors;
pub use stock_management_errors::ManageStockError;

mod payment_errors;
pub use payment_errors::PaymentIntentError;

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}
impl ErrorResponse {
    pub fn json(msg: String) -> Json<ErrorResponse> {
        Json(ErrorResponse { error: msg })
    }
}
