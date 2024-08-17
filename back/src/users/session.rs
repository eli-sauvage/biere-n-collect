use crate::errors::{GetSessionError, ServerError};

use rocket::time::Duration;
use sqlx::{types::time::OffsetDateTime, MySql, Pool};
use std::str::FromStr;
use uuid::Uuid;

const SESSION_DURATION: Duration = Duration::hours(12);

#[derive(Clone, Debug)]
pub struct Session {
    pub email: String,
    pub expires: OffsetDateTime,
    pub uuid: Uuid,
}
impl Session {
    async fn delete_old_sessions(pool: &Pool<MySql>) -> Result<(), ServerError> {
        sqlx::query!("DELETE FROM Sessions WHERE CURRENT_TIMESTAMP > expires")
            .execute(pool)
            .await
            .map_err(ServerError::Sqlx)?;
        Ok(())
    }

    pub async fn delete_if_exists(pool: &Pool<MySql>, email: &str) -> Result<(), ServerError> {
        sqlx::query!(
            "DELETE FROM Sessions WHERE user_id = (SELECT id FROM Users WHERE email = ?)",
            email
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn new(pool: &Pool<MySql>, email: String) -> Result<Session, ServerError> {
        Session::delete_old_sessions(pool).await?;
        Session::delete_if_exists(pool, &email).await?;
        let session = Session {
            uuid: Uuid::new_v4(),
            expires: OffsetDateTime::now_utc() + SESSION_DURATION,
            email,
        };

        sqlx::query!(
            "INSERT INTO Sessions (user_id, expires, uuid) VALUES ((SELECT id FROM Users WHERE email = ?), ?, ?)",
            session.email,
            session.expires,
            session.uuid.to_string()
        ).execute(pool)
        .await
        .map_err(ServerError::Sqlx)?;

        Ok(session)
    }

    pub async fn get_from_email(
        pool: &Pool<MySql>,
        email: &str,
    ) -> Result<Result<Session, GetSessionError>, ServerError> {
        Session::delete_old_sessions(pool).await?;
        let result = match sqlx::query!(
                    "SELECT uuid, expires FROM Sessions WHERE user_id = (SELECT id FROM Users WHERE email = ?)",
                    email
                )
                .fetch_optional(pool)
                .await
                .map_err(ServerError::Sqlx)? {
            Some(res) => {res},
            None => {return Ok(Err(GetSessionError::NoSession(email.to_owned())))}};
        let uuid = Uuid::from_str(&result.uuid)?;
        let expires = result.expires;

        Ok(Ok(Session {
            email: email.to_owned(),
            expires,
            uuid,
        }))
    }
}
