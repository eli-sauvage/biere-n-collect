use crate::{db, errors::ServerError};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Category {
    pub id: u32,
    pub name: String,
}

impl Category {
    pub async fn create(name: String) -> Result<Category, ServerError> {
        let id = sqlx::query!("INSERT INTO Categories (name) VALUES (?)", name)
            .execute(db())
            .await?
            .last_insert_id() as u32;
        Ok(Category { id, name })
    }
    pub async fn get(id: u32) -> Result<Option<Category>, ServerError> {
        let res = sqlx::query_as!(Category, "SELECT id, name FROM Categories WHERE id = ?", id)
            .fetch_optional(db())
            .await?;
        Ok(res)
    }
    pub async fn delete(self) -> Result<(), ServerError> {
        sqlx::query!(
            "UPDATE Products SET category_id = NULL WHERE category_id = ?",
            self.id
        )
        .execute(db())
        .await?;

        sqlx::query!("DELETE FROM Categories WHERE id = ?", self.id)
            .execute(db())
            .await?;

        Ok(())
    }
    pub async fn set_name(&mut self, new_name: String) -> Result<(), ServerError> {
        sqlx::query!(
            "UPDATE Categories SET name = ? WHERE id = ?",
            new_name,
            self.id
        )
        .execute(db())
        .await?;
        self.name = new_name;
        Ok(())
    }
}
pub async fn get_all() -> Result<Vec<Category>, ServerError> {
    let res = sqlx::query_as!(Category, "SELECT id, name FROM Categories")
        .fetch_all(db())
        .await?;
    Ok(res)
}
