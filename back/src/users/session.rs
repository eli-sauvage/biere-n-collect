use crate::{
    errors::{EndSessionError, ServerError},
    users::user::{ErrorMsg, User},
};

use rocket::{
    http::CookieJar,
    serde::json::{json, Json, Value},
    time::Duration,
    State,
};
use serde::{ser::SerializeStruct, Serialize};
use sqlx::{types::time::OffsetDateTime, MySql, Pool};
use uuid::Uuid;

const SESSION_DURATION: Duration = Duration::hours(12);

#[derive(Clone, Debug)]
pub struct Session {
    pub email: String,
    pub expires: OffsetDateTime,
    pub uuid: String,
}
impl Session {
    async fn delete_old_sessions(pool: &Pool<MySql>) -> Result<(), ServerError> {
        sqlx::query!("DELETE FROM Sessions WHERE CURRENT_TIMESTAMP > expires")
            .execute(pool)
            .await
            .map_err(ServerError::Sqlx)?;
        Ok(())
    }

    pub async fn delete_if_exists(pool: &Pool<MySql>, uuid: &str) -> Result<(), ServerError> {
        sqlx::query!("DELETE FROM Sessions WHERE uuid = ?", uuid)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn new(pool: &Pool<MySql>, email: String) -> Result<Session, ServerError> {
        Session::delete_old_sessions(pool).await?;
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
        ).execute(pool)
        .await
        .map_err(ServerError::Sqlx)?;

        Ok(session)
    }

    pub async fn get_all(pool: &Pool<MySql>) -> Result<Vec<Session>, ServerError> {
        let sessions = sqlx::query_as!(Session, "SELECT email, expires, uuid FROM Sessions INNER JOIN Users ON Users.id = Sessions.user_id")
            .fetch_all(pool)
            .await?;

        Ok(sessions)
    }

    pub async fn get_all_sessions_for_email(
        pool: &Pool<MySql>,
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

impl Serialize for Session {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Session", 3)?;
        state.serialize_field("email", &self.email)?;
        state.serialize_field("expires", &self.expires.to_string())?;
        state.serialize_field("uuid", &self.uuid)?;
        state.end()
    }
}

#[get("/get_auth")]
pub async fn get_auth(user: Option<User>) -> Json<Value> {
    if let Some(user) = user {
        Json(json!({"authenticated": true, "role": user.role.to_string(), "email": user.email}))
    } else {
        Json(json!({"authenticated": false}))
    }
}

#[post("/end")]
pub async fn end_sessions(
    pool: &State<Pool<MySql>>,
    cookie: &CookieJar<'_>,
    user: Result<User, ErrorMsg>,
) -> Result<Json<Value>, EndSessionError> {
    if let Err(_) = user {
        return Err(EndSessionError::UserNotFound);
    }
    let session = cookie
        .get("session")
        .ok_or_else(|| EndSessionError::UserNotFound)?
        .to_string();

    cookie.remove("session");

    Session::delete_if_exists(pool, &session).await?;

    Ok(Json(json!({"success": true})))
}
