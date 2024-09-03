use crate::{db, errors::ServerError};

use sqlx::types::time::OffsetDateTime;
use std::time::Duration;

use uuid::Uuid;

const SESSION_DURATION: Duration = Duration::from_secs(12 * 60 * 60);

#[derive(Clone, Debug)]
pub struct Session {
    pub email: String,
    pub expires: OffsetDateTime,
    pub uuid: String,
}
impl Session {
    async fn delete_old_sessions() -> Result<(), ServerError> {
        sqlx::query!("DELETE FROM Sessions WHERE CURRENT_TIMESTAMP > expires")
            .execute(db())
            .await
            .map_err(ServerError::Sqlx)?;
        Ok(())
    }

    pub async fn delete_if_exists(uuid: &str) -> Result<(), ServerError> {
        sqlx::query!("DELETE FROM Sessions WHERE uuid = ?", uuid)
            .execute(db())
            .await?;
        Ok(())
    }

    pub async fn new(email: String) -> Result<Session, ServerError> {
        Session::delete_old_sessions().await?;
        // Session::delete_if_exists(pool, &email).await?;
        let session = Session {
            uuid: Uuid::new_v4().to_string(),
            expires: OffsetDateTime::now_utc() + SESSION_DURATION,
            email,
        };

        sqlx::query!(
            "INSERT INTO Sessions (user_id, expires, uuid) VALUES ((SELECT id FROM Users WHERE email = ?), ?, ?)",
            session.email,
            session.expires,
            session.uuid
        ).execute(db())
        .await
        .map_err(ServerError::Sqlx)?;

        Ok(session)
    }

    pub async fn get_all() -> Result<Vec<Session>, ServerError> {
        let sessions = sqlx::query_as!(Session, "SELECT email, expires, uuid FROM Sessions INNER JOIN Users ON Users.id = Sessions.user_id")
            .fetch_all(db())
            .await?;

        Ok(sessions)
    }

    pub async fn get_all_sessions_for_email(email: &str) -> Result<Vec<Session>, ServerError> {
        Session::delete_old_sessions().await?;
        let sessions = sqlx::query_as!(
            Session,
                "SELECT uuid, expires, email FROM Sessions INNER JOIN Users ON Sessions.user_id = Users.id WHERE Users.email = ?",
                email
            )
            .fetch_all(db())
            .await
            .map_err(ServerError::Sqlx)?;
        Ok(sessions)
    }
}
