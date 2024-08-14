use crate::errors::Error;
use sqlx::{types::time::OffsetDateTime, MySql, Pool};

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct Order {
    id: u32,
    timestamp: OffsetDateTime,
    validated: bool,
    user_email: String,
    receipt: Option<String>,
}
impl Order {
    pub async fn from_id(pool: &Pool<MySql>, id: u32) -> Result<Option<Order>, Error> {
        sqlx::query_as!(
            Order,
            "SELECT id, timestamp, validated as \"validated!: bool\", user_email, receipt from Orders WHERE id = ?",
            id
        )
        .fetch_optional(pool).await.map_err(Error::Sqlx)
    }
}

pub struct OrderDetails {
    id: u32,
    order_id: u32,
    product_id: u32,
    quantity: u32,
}

impl Order {
    pub async fn get_full_price(&self, pool: &Pool<MySql>) -> Result<i64, Error> {
        let total = sqlx::query!(
            "SELECT cast(SUM(price * quantity) as int) as result from OrderDetails INNER JOIN ProductTypes ON OrderDetails.product_id = ProductTypes.id WHERE order_id = ? ;",
            self.id
        ).fetch_one(pool).await.map_err(Error::Sqlx)?;

        total.result.ok_or(Error::Sqlx(sqlx::Error::RowNotFound))
    }
}
