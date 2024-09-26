use crate::errors::{ServerError, SessionError, UserParseError};

use sqlx::{types::time::OffsetDateTime, MySqlPool};
use std::time::Duration;

use uuid::Uuid;

use super::user::User;

const SESSION_DURATION: Duration = Duration::from_secs(12 * 60 * 60);

#[derive(Clone, Debug)]
pub struct Session {
    pub email: String,
    pub expires: OffsetDateTime,
    pub uuid: String,
}
impl Session {
    async fn delete_old_sessions(pool: &MySqlPool) -> Result<(), ServerError> {
        sqlx::query!("DELETE FROM Sessions WHERE CURRENT_TIMESTAMP > expires")
            .execute(pool)
            .await
            .map_err(ServerError::Sqlx)?;
        Ok(())
    }

    pub async fn delete_if_exists(pool: &MySqlPool, uuid: &str) -> Result<(), ServerError> {
        sqlx::query!("DELETE FROM Sessions WHERE uuid = ?", uuid)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn new(pool: &MySqlPool, email: String) -> Result<Session, SessionError> {
        Session::delete_old_sessions(pool).await?;
        // Session::delete_if_exists(pool, &email).await?;
        let session = Session {
            uuid: Uuid::new_v4().to_string(),
            expires: OffsetDateTime::now_utc() + SESSION_DURATION,
            email,
        };

        let user = User::get_from_email(pool, &session.email)
            .await?
            .ok_or(SessionError::UserNotFound(session.email.clone()))?;

        sqlx::query!(
            "INSERT INTO Sessions (user_id, expires, uuid) VALUES (?, ?, ?)",
            user.id,
            session.expires,
            session.uuid
        )
        .execute(pool)
        .await
        .map_err(ServerError::Sqlx)?;

        Ok(session)
    }

    pub async fn get_all(pool: &MySqlPool) -> Result<Vec<Session>, ServerError> {
        let sessions = sqlx::query_as!(Session, "SELECT email, expires, uuid FROM Sessions INNER JOIN Users ON Users.id = Sessions.user_id")
            .fetch_all(pool)
            .await?;

        Ok(sessions)
    }

    pub async fn get_all_sessions_for_email(
        pool: &MySqlPool,
        email: &str,
    ) -> Result<Vec<Session>, ServerError> {
        Session::delete_old_sessions(pool).await?;
        let sessions = sqlx::query_as!(
            Session,
                "SELECT uuid, expires, email FROM Sessions INNER JOIN Users ON Sessions.user_id = Users.id WHERE Users.email = ?",
                email
            )
            .fetch_all(pool)
            .await
            .map_err(ServerError::Sqlx)?;
        Ok(sessions)
    }
}
