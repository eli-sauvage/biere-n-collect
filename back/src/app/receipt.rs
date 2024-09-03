use std::ops::Deref;

use qrcode::QrCode;
use serde::Serialize;

use crate::errors::ServerError;
#[derive(Serialize, Debug, Clone, sqlx::Decode)]
pub struct Receipt(pub String);

impl Receipt {
    pub fn get_qr_code(&self) -> Result<QrCode, ServerError> {
        let qr = QrCode::with_version(self, qrcode::Version::Normal(5), qrcode::EcLevel::H)
            .map_err(ServerError::QrCode)?;
        Ok(qr)
    }
}
impl AsRef<[u8]> for Receipt {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
impl From<String> for Receipt {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl Deref for Receipt {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
