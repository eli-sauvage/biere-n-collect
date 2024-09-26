use crate::errors::ServerError;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

use crate::app::product_variations::Variation;

#[derive(Debug, Serialize)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub stock_quantity: f32,
    pub variations: Vec<Variation>,
}

impl Product {
    pub async fn create(
        pool: &MySqlPool,
        name: String,
        description: String,
        stock_quantity: f32,
    ) -> Result<Product, ServerError> {
        //shift every product down
        sqlx::query!("UPDATE Products set position = position + 1")
            .execute(pool)
            .await?;

        let id = sqlx::query!(
            "
            INSERT INTO Products (name, description, stock_quantity, position)
            VALUES (?, ?, ?, 0)",
            name,
            description,
            stock_quantity,
        )
        .execute(pool)
        .await?
        .last_insert_id() as u32;

        Ok(Product {
            id,
            name,
            description,
            stock_quantity,
            variations: vec![],
        })
    }
    pub async fn get(pool: &MySqlPool, id: u32) -> Result<Option<Product>, ServerError> {
        let res_prod = sqlx::query!(
            "SELECT id, name, description, stock_quantity FROM Products WHERE id = ?",
            id
        )
        .fetch_optional(pool)
        .await?;
        if let Some(prod) = res_prod {
            let variations = sqlx::query_as!(
                Variation,
                "SELECT id, name, price_ht, tva, volume, product_id,
                available_to_order as \"available_to_order: bool\"
                FROM ProductVariations WHERE product_id = ?",
                prod.id
            )
            .fetch_all(pool)
            .await?;
            Ok(Some(Product {
                id: prod.id,
                name: prod.name,
                description: prod.description,
                stock_quantity: prod.stock_quantity,
                variations,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(self, pool: &MySqlPool) -> Result<(), ServerError> {
        for variation in self.variations {
            variation.delete(pool).await?;
        }
        sqlx::query!("DELETE FROM Products WHERE id = ?", self.id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn get_position(&self, pool: &MySqlPool) -> Result<u16, ServerError> {
        let pos = sqlx::query!("SELECT position FROM Products WHERE id = ?", self.id)
            .fetch_one(pool)
            .await?
            .position;
        Ok(pos.unwrap_or(0))
    }
    pub async fn set_name(
        &mut self,
        pool: &MySqlPool,
        new_name: String,
    ) -> Result<(), ServerError> {
        sqlx::query!(
            "UPDATE Products SET name = ? WHERE id = ?",
            new_name,
            self.id
        )
        .execute(pool)
        .await?;
        self.name = new_name;
        Ok(())
    }
    pub async fn set_description(
        &mut self,
        pool: &MySqlPool,
        new_description: String,
    ) -> Result<(), ServerError> {
        sqlx::query!(
            "UPDATE Products SET description = ? WHERE id = ?",
            new_description,
            self.id
        )
        .execute(pool)
        .await?;
        self.description = new_description;
        Ok(())
    }
    pub async fn set_stock_quantity(
        &mut self,
        pool: &MySqlPool,
        new_stock_quantity: f32,
    ) -> Result<(), ServerError> {
        sqlx::query!(
            "UPDATE Products SET stock_quantity = ? WHERE id = ?",
            new_stock_quantity,
            self.id
        )
        .execute(pool)
        .await?;
        self.stock_quantity = new_stock_quantity;
        Ok(())
    }

    pub async fn add_variation(
        &mut self,
        pool: &MySqlPool,
        name: String,
        price_ht: i32,
        tva: f32,
        volume: f32,
        available_to_order: bool,
    ) -> Result<(), ServerError> {
        let variation_id = sqlx::query!(
            "INSERT INTO ProductVariations (name, product_id, price_ht, tva, volume, available_to_order)
            VALUES (?, ?, ?, ?, ?, ?)",
            name,
            self.id,
            price_ht,
            tva,
            volume,
            available_to_order
        )
        .execute(pool)
        .await?
        .last_insert_id() as u32;

        self.variations.push(Variation {
            id: variation_id,
            name,
            price_ht,
            tva,
            product_id: self.id,
            volume,
            available_to_order,
        });

        Ok(())
    }

    pub async fn delete_variation(
        &mut self,
        pool: &MySqlPool,
        variation_id: u32,
    ) -> Result<(), ServerError> {
        if let Some(variation_index) = self.variations.iter().position(|v| v.id == variation_id) {
            let variation = self.variations.remove(variation_index);
            sqlx::query!("DELETE FROM ProductVariations WHERE id = ?", variation.id)
                .execute(pool)
                .await?;
        }

        Ok(())
    }
}

#[derive(Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MoveDirection {
    Up,
    Down,
}

impl Product {
    pub async fn move_product(
        &mut self,
        pool: &MySqlPool,
        direction: MoveDirection,
    ) -> Result<(), ServerError> {
        let max_pos = sqlx::query!("SELECT MAX(position) as max_pos FROM Products",)
            .fetch_one(pool)
            .await?
            .max_pos
            .unwrap_or(0);
        let current_position = self.get_position(pool).await?;
        let new_pos = match (current_position, direction) {
            (0, MoveDirection::Up) => current_position,
            (pos, MoveDirection::Down) if pos == max_pos => current_position,
            (_, MoveDirection::Up) => current_position - 1,
            (_, MoveDirection::Down) => current_position + 1,
        };

        let mut transaction = pool.begin().await?;
        sqlx::query!("UPDATE Products SET position = NULL WHERE id = ?", self.id)
            .execute(&mut *transaction)
            .await?;
        sqlx::query!(
            "UPDATE Products SET position = ? WHERE position = ?",
            current_position,
            new_pos,
        )
        .execute(&mut *transaction)
        .await?;
        sqlx::query!(
            "UPDATE Products SET position = ? WHERE id = ?",
            new_pos,
            self.id
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;
        Ok(())
    }
}

pub async fn get_all(pool: &MySqlPool) -> Result<Vec<Product>, ServerError> {
    let prods = sqlx::query!(
        "SELECT id, name, description, stock_quantity
        FROM Products ORDER BY position"
    )
    .fetch_all(pool)
    .await?;
    let mut res: Vec<Product> = vec![];
    for prod in prods {
        let variations = sqlx::query_as!(
            Variation,
            "SELECT id, name, price_ht, tva, volume, product_id, 
            available_to_order as \"available_to_order: bool\"
            FROM ProductVariations WHERE product_id = ?",
            prod.id
        )
        .fetch_all(pool)
        .await?;
        res.push(Product {
            id: prod.id,
            name: prod.name,
            description: prod.description,
            stock_quantity: prod.stock_quantity,
            variations,
        });
    }
    Ok(res)
}
