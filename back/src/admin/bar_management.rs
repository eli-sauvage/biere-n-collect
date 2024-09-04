use serde::Serialize;
use sqlx::types::time::OffsetDateTime;

use crate::{db, errors::ServerError, utils::serialize_time};

#[derive(Serialize)]
pub struct Bar {
    pub is_open: bool,
    #[serde(serialize_with = "serialize_time")]
    pub open_since: OffsetDateTime,
    pub closing_message: String,
}
impl Bar {
    pub async fn get() -> Result<Bar, ServerError> {
        let res = sqlx::query_as!(
            Bar,
            "SELECT is_open as \"is_open: bool\", open_since, closing_message FROM Bar"
        )
        .fetch_one(db())
        .await?;
        Ok(res)
    }

    pub async fn open(&mut self) -> Result<(), ServerError> {
        sqlx::query!("UPDATE Bar SET is_open = TRUE")
            .execute(db())
            .await?;
        let now = OffsetDateTime::now_utc();
        sqlx::query!("UPDATE Bar SET open_since = ?", now)
            .execute(db())
            .await?;
        self.is_open = true;
        self.open_since = now;
        Ok(())
    }

    pub async fn close(&mut self) -> Result<(), ServerError> {
        sqlx::query!("UPDATE Bar SET is_open = FALSE")
            .execute(db())
            .await?;
        self.is_open = false;
        //#TODO: genete report
        Ok(())
    }

    pub async fn set_closing_message(&mut self, msg: &str) -> Result<(), ServerError> {
        sqlx::query!("UPDATE Bar SET closing_message = ?", msg)
            .execute(db())
            .await?;
        Ok(())
    }
}

pub struct PdfReport {}
pub async fn generate_report() -> Result<PdfReport, ServerError> {
    todo!()
}
