use serde::{Deserialize, Serialize};

pub type PaymentIntentId = String;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentStatus {
    Canceled,
    Processing,
    RequiresAction,
    RequiresCapture,
    RequiresConfirmation,
    RequiresPaymentMethod,
    Succeeded,
}

#[derive(Deserialize, Debug)]
pub struct PaymentIntent {
    pub id: PaymentIntentId,
    pub client_secret: String,
    pub status: PaymentIntentStatus,
    pub amount: i32,
}
