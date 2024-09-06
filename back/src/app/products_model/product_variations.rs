use crate::{db, errors::ServerError};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Variation {
    pub id: u32,
    pub name: String,
    pub product_id: u32,
    pub price_ht: i32,
    pub tva: f32,
    pub volume: f32,
}

impl Variation {
    pub async fn get(id: u32) -> Result<Option<Variation>, ServerError> {
        let res = sqlx::query_as!(
            Variation,
            "SELECT id, name, price_ht, tva, product_id, volume FROM ProductVariations WHERE id = ?",
            id
        )
        .fetch_optional(db())
        .await?;
        Ok(res)
    }

    //pub async fn get_product(&self) -> Result<Product, ServerError> {
    //    Product::get(self.product_id)
    //        .await
    //        .map(|p| p.expect("product not found from one of its variations"))
    //}

    pub async fn delete(self) -> Result<(), ServerError> {
        sqlx::query!("DELETE FROM ProductVariations WHERE id = ?", self.id)
            .execute(db())
            .await?;
        Ok(())
    }

    pub async fn set_price_ht(&mut self, new_price_ht: i32) -> Result<(), ServerError> {
        sqlx::query!(
            "UPDATE ProductVariations SET price_ht = ? WHERE id = ?",
            new_price_ht,
            self.id
        )
        .execute(db())
        .await?;
        self.price_ht = new_price_ht;
        Ok(())
    }
    pub async fn set_name(&mut self, new_name: String) -> Result<(), ServerError> {
        sqlx::query!(
            "UPDATE ProductVariations SET name = ? WHERE id = ?",
            new_name,
            self.id
        )
        .execute(db())
        .await?;
        self.name = new_name;
        Ok(())
    }
    pub async fn set_volume(&mut self, new_volume: f32) -> Result<(), ServerError> {
        sqlx::query!(
            "UPDATE ProductVariations SET volume = ? WHERE id = ?",
            new_volume,
            self.id
        )
        .execute(db())
        .await?;
        self.volume = new_volume;
        Ok(())
    }
}
