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
        msg: String,
    ) -> Result<(), ServerError> {
        sqlx::query!("UPDATE Bar SET closing_message = ?", msg)
            .execute(pool)
            .await?;
        self.closing_message = msg;
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

#[sqlx::test]
async fn test_bar_open_close(pool: MySqlPool) {
    let mut bar = Bar::get(&pool).await.unwrap();
    bar.open(&pool).await.unwrap();
    assert!(bar.is_open);
    assert!(
        sqlx::query!("SELECT is_open as \"is_open: bool\" FROM Bar")
            .fetch_one(&pool)
            .await
            .unwrap()
            .is_open
    );
    bar.close(&pool).await.unwrap();
    assert!(!bar.is_open);
    assert!(
        !sqlx::query!("SELECT is_open as \"is_open: bool\" FROM Bar")
            .fetch_one(&pool)
            .await
            .unwrap()
            .is_open
    );
}

#[sqlx::test]
async fn test_closing_message(pool: MySqlPool) {
    let mut bar = Bar::get(&pool).await.unwrap();
    let messages = vec![
        "bar fermé",
        "le bar est fermé\nmultiple lignes",
        "le bar est femeé\nspecial character ❌",
    ];
    for message in messages {
        bar.set_closing_message(&pool, message.to_string())
            .await
            .unwrap();
        assert_eq!(bar.closing_message, message);
        assert_eq!(
            sqlx::query!("SELECT closing_message FROM Bar")
                .fetch_one(&pool)
                .await
                .unwrap()
                .closing_message,
            message
        );
    }
}

#[sqlx::test]
async fn test_get_openings(pool: MySqlPool) {
    use std::time::Duration;

    let mut bar = Bar::get(&pool).await.unwrap();
    bar.open(&pool).await.unwrap();
    bar.close(&pool).await.unwrap();
    bar.open(&pool).await.unwrap();
    std::thread::sleep(Duration::from_secs(1));
    bar.close(&pool).await.unwrap();

    let openings = get_bar_openings(&pool).await.unwrap();
    assert_eq!(openings.len(), 2);
    assert!(openings[1].end > openings[1].begin);
}
