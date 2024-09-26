use serde::Serialize;
use sqlx::{types::time::OffsetDateTime, MySqlPool};

use crate::{errors::ServerError, utils::serialize_time};

#[derive(Serialize)]
pub struct Bar {
    pub is_open: bool,
    #[serde(serialize_with = "serialize_time")]
    pub open_since: OffsetDateTime,
    pub closing_message: String,
}
impl Bar {
    pub async fn get(pool: &MySqlPool) -> Result<Bar, ServerError> {
        let res = sqlx::query_as!(
            Bar,
            "SELECT is_open as \"is_open: bool\", open_since, closing_message FROM Bar"
        )
        .fetch_one(pool)
        .await?;
        Ok(res)
    }

    pub async fn open(&mut self, pool: &MySqlPool) -> Result<(), ServerError> {
        sqlx::query!("UPDATE Bar SET is_open = TRUE")
            .execute(pool)
            .await?;
        let now = OffsetDateTime::now_utc();
        sqlx::query!("UPDATE Bar SET open_since = ?", now)
            .execute(pool)
            .await?;
        self.is_open = true;
        self.open_since = now;
        Ok(())
    }

    pub async fn close(&mut self, pool: &MySqlPool) -> Result<(), ServerError> {
        sqlx::query!("UPDATE Bar SET is_open = FALSE")
            .execute(pool)
            .await?;
        sqlx::query!(
            "INSERT INTO BarOpenings (begin, end) VALUES (?, CURRENT_TIMESTAMP)",
            self.open_since
        )
        .execute(pool)
        .await?;
        self.is_open = false;
        Ok(())
    }

    pub async fn set_closing_message(
        &mut self,
        pool: &MySqlPool,
        msg: &str,
    ) -> Result<(), ServerError> {
        sqlx::query!("UPDATE Bar SET closing_message = ?", msg)
            .execute(pool)
            .await?;
        Ok(())
    }
}

#[derive(Serialize)]
pub struct BarOpening {
    #[serde(serialize_with = "serialize_time")]
    begin: OffsetDateTime,
    #[serde(serialize_with = "serialize_time")]
    end: OffsetDateTime,
}
pub async fn get_bar_openings(pool: &MySqlPool) -> Result<Vec<BarOpening>, ServerError> {
    let res = sqlx::query!("SELECT begin, end FROM BarOpenings")
        .fetch_all(pool)
        .await?;
    Ok(res
        .into_iter()
        .map(|r| BarOpening {
            begin: r.begin,
            end: r.end,
        })
        .collect())
}
