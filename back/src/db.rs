use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::env;

pub async fn setup_db_and_migrate() -> Pool<MySql> {
    let db_password = env::var("MARIADB_PASSWORD").expect("db password is not set in environment");
    let db_host = env::var("MARIADB_HOST").expect("mariadb host is not set in environment");

    let pool = match MySqlPoolOptions::new()
        .max_connections(20)
        .connect(format!("mysql://app:{db_password}@{db_host}:3306/lhavrais-pay").as_str())
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

    pool
}
