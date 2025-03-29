use core::fmt;
use serde::{de, Deserialize, Deserializer, Serializer};
use sqlx::{migrate, sqlite::SqlitePoolOptions, types::time::OffsetDateTime, SqlitePool};
use std::{env, str::FromStr};

pub static MIGRATOR: migrate::Migrator = sqlx::migrate!("./migrations");

pub async fn setup_db_and_migrate() -> SqlitePool {
    for env_name in [
        "DATABASE_URL",
        "SMTP_USERNAME",
        "SMTP_PASSWORD",
        "SMTP_SERVER",
        "VITE_BAR_NAME",
    ] {
        if env::var(env_name)
            .expect(&format!("env var {env_name} not found"))
            .is_empty()
        {
            panic!("env var {env_name} is empty");
        }
    }
    let db_url = env::var("DATABASE_URL").unwrap();

    let pool = match SqlitePoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
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

    if let Err(e) = MIGRATOR.run(&pool).await {
        panic!("could not migrate : {e:?}");
    };

    pool
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
