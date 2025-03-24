use core::fmt;
use serde::{de, Deserialize, Deserializer, Serializer};
use sqlx::{migrate, mysql::MySqlPoolOptions, types::time::OffsetDateTime, MySqlPool};
use std::{env, str::FromStr};

pub static MIGRATOR: migrate::Migrator = sqlx::migrate!("./migrations");

pub async fn setup_db_and_migrate() -> MySqlPool {
    for env_name in [
        "MARIADB_PASSWORD",
        "MARIADB_HOST",
        "SMTP_USERNAME",
        "SMTP_PASSWORD",
        "VITE_BAR_NAME",
    ] {
        if env::var(env_name)
            .expect("env var {env_name} not found")
            .is_empty()
        {
            panic!("env var {env_name} is empty");
        }
    }
    let db_password = env::var("MARIADB_PASSWORD").expect("db password is not set in environment");
    let db_host = env::var("MARIADB_HOST").expect("mariadb host is not set in environment");

    let pool = match MySqlPoolOptions::new()
        .max_connections(20)
        .connect(format!("mysql://app:{db_password}@{db_host}:3306/biere-n-collect").as_str())
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
