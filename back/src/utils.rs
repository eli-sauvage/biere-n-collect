use core::fmt;
use serde::{de, Deserialize, Deserializer, Serializer};
use sqlx::{mysql::MySqlPoolOptions, types::time::OffsetDateTime, MySql, Pool};
use std::{env, str::FromStr};
use tokio::sync::OnceCell;

static DB: OnceCell<Pool<MySql>> = OnceCell::const_new();

pub fn db() -> &'static Pool<MySql> {
    DB.get().expect("db oncecell is not initialized")
}

pub async fn setup_db_and_migrate() {
    let db_password = env::var("MARIADB_PASSWORD").expect("db password is not set in environment");
    let db_host = env::var("MARIADB_HOST").expect("mariadb host is not set in environment");

    env::var("SMTP_USERNAME").expect("env var SMTP_USERNAME not found");
    env::var("SMTP_PASSWORD").expect("env var SMTP_PASSWORD not found");
    env::var("VITE_BAR_NAME").expect("env var VITE_BAR_NAME not found");

    let pool = match MySqlPoolOptions::new()
        .max_connections(20)
        .connect(format!("mysql://app:{db_password}@{db_host}:3306/beer-n-collect").as_str())
        .await
    {
        Ok(pool) => pool,
        Err(e) => panic!("could not connect to db : {e:?}"),
    };
    let row: (i64,) = sqlx::query_as("SELECT 150")
        .fetch_one(&pool)
        .await
        .expect("could not execute test query");
    assert_eq!(row.0, 150); //test connection

    if let Err(e) = sqlx::migrate!("./migrations").run(&pool).await {
        panic!("could not migrate : {e:?}");
    };

    DB.get_or_init(|| async move { pool }).await;
}

pub fn serialize_time<S: Serializer>(
    dt: &OffsetDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let time = dt.unix_timestamp() * 1000;
    serializer.serialize_i64(time)
}

pub fn deserialize_empty_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}
