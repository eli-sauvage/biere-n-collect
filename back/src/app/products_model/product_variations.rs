use crate::errors::ServerError;
use serde::Serialize;
use sqlx::SqlitePool;

#[derive(Debug, Serialize)]
pub struct Variation {
    pub id: u32,
    pub name: String,
    pub product_id: u32,
    pub price_ht: i32,
    pub tva: f32,
    pub volume: f32,
    pub available_to_order: bool,
}

impl Variation {
    pub async fn get(pool: &SqlitePool, id: u32) -> Result<Option<Variation>, ServerError> {
        let res = sqlx::query!("SELECT * FROM ProductVariations WHERE id = ?", id)
            .fetch_optional(pool)
            .await?
            .map(|r| Variation {
                id: r.id as u32,
                name: r.name,
                product_id: r.product_id as u32,
                price_ht: r.price_ht as i32,
                tva: r.tva as f32,
                volume: r.volume as f32,
                available_to_order: r.available_to_order,
            });
        Ok(res)
    }
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Variation>, ServerError> {
        let res = sqlx::query!("SELECT * FROM ProductVariations")
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|r| Variation {
                id: r.id as u32,
                name: r.name,
                product_id: r.product_id as u32,
                price_ht: r.price_ht as i32,
                tva: r.tva as f32,
                volume: r.volume as f32,
                available_to_order: r.available_to_order,
            });
        Ok(res.collect())
    }

    pub async fn delete(self, pool: &SqlitePool) -> Result<(), ServerError> {
        sqlx::query!("DELETE FROM ProductVariations WHERE id = ?", self.id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn set_price_ht(
        &mut self,
        pool: &SqlitePool,
        new_price_ht: i32,
    ) -> Result<(), ServerError> {
        sqlx::query!(
            "UPDATE ProductVariations SET price_ht = ? WHERE id = ?",
            new_price_ht,
            self.id
        )
        .execute(pool)
        .await?;
        self.price_ht = new_price_ht;
        Ok(())
    }

    pub async fn set_tva(&mut self, pool: &SqlitePool, new_tva: f32) -> Result<(), ServerError> {
        sqlx::query!(
            "UPDATE ProductVariations SET tva = ? WHERE id = ?",
            new_tva,
            self.id
        )
        .execute(pool)
        .await?;
        self.tva = new_tva;
        Ok(())
    }

    pub async fn set_name(
        &mut self,
        pool: &SqlitePool,
        new_name: String,
    ) -> Result<(), ServerError> {
        sqlx::query!(
            "UPDATE ProductVariations SET name = ? WHERE id = ?",
            new_name,
            self.id
        )
        .execute(pool)
        .await?;
        self.name = new_name;
        Ok(())
    }
    pub async fn set_volume(
        &mut self,
        pool: &SqlitePool,
        new_volume: f32,
    ) -> Result<(), ServerError> {
        sqlx::query!(
            "UPDATE ProductVariations SET volume = ? WHERE id = ?",
            new_volume,
            self.id
        )
        .execute(pool)
        .await?;
        self.volume = new_volume;
        Ok(())
    }
    pub async fn set_available_to_order(
        &mut self,
        pool: &SqlitePool,
        new_available_to_order: bool,
    ) -> Result<(), ServerError> {
        sqlx::query!(
            "UPDATE ProductVariations SET available_to_order = ? WHERE id = ?",
            new_available_to_order,
            self.id
        )
        .execute(pool)
        .await?;
        self.available_to_order = new_available_to_order;
        Ok(())
    }
}
