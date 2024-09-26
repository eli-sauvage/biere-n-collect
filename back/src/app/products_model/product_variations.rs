use crate::errors::ServerError;
use serde::Serialize;
use sqlx::MySqlPool;

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
    pub async fn get(pool: &MySqlPool, id: u32) -> Result<Option<Variation>, ServerError> {
        let res = sqlx::query_as!(
            Variation,
            "SELECT id, name, price_ht, tva, product_id, volume, available_to_order as \"available_to_order: bool\" FROM ProductVariations WHERE id = ?",
            id
        )
        .fetch_optional(pool)
        .await?;
        Ok(res)
    }
    pub async fn get_all(pool: &MySqlPool) -> Result<Vec<Variation>, ServerError> {
        let res = sqlx::query_as!(
            Variation,
            "SELECT id, name, price_ht, tva, product_id, volume, available_to_order as \"available_to_order: bool\" FROM ProductVariations"
        ).fetch_all(pool)
            .await?;
        Ok(res)
    }

    pub async fn delete(self, pool: &MySqlPool) -> Result<(), ServerError> {
        sqlx::query!("DELETE FROM ProductVariations WHERE id = ?", self.id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn set_price_ht(
        &mut self,
        pool: &MySqlPool,
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

    pub async fn set_tva(&mut self, pool: &MySqlPool, new_tva: f32) -> Result<(), ServerError> {
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
        pool: &MySqlPool,
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
        pool: &MySqlPool,
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
        pool: &MySqlPool,
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
