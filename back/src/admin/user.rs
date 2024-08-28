use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize, Serializer};

use crate::{
    db,
    errors::{ServerError, UserParseError},
};

use super::auth::Session;

#[derive(Eq, PartialEq, Hash, Clone, Debug, Serialize, Deserialize, sqlx::Type, Copy)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum Role {
    Admin,
    Waiter,
}

#[derive(Clone, Debug, Serialize)]
pub struct User {
    pub email: String,
    pub role: Role,
    #[serde(serialize_with = "serialize_sessions_into_len")]
    pub active_sessions: Vec<Session>,
}
fn serialize_sessions_into_len<S>(sessions: &[Session], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u32(sessions.len() as u32)
}

impl User {
    pub async fn create(email: &str, role: Role) -> Result<User, ServerError> {
        sqlx::query!("INSERT INTO Users (email, role) VALUES (?, ?)", email, role)
            .execute(db())
            .await
            .map_err(ServerError::Sqlx)?;
        Ok(User {
            email: email.to_owned(),
            role,
            active_sessions: vec![],
        })
    }

    pub async fn get_all() -> Result<Vec<User>, ServerError> {
        let record = sqlx::query!("SELECT email, role as \"role: Role\" FROM Users")
            .fetch_all(db())
            .await?;
        let all_sessions = Session::get_all().await?;
        let users: Vec<User> = record
            .into_iter()
            .map(|r| {
                let sessions: Vec<Session> = all_sessions
                    .iter()
                    .filter(|s| s.email == r.email)
                    .cloned()
                    .collect();
                User {
                    email: r.email,
                    role: r.role,
                    active_sessions: sessions,
                }
            })
            .collect();
        Ok(users)
    }

    pub async fn get_from_email(email: &str) -> Result<Option<User>, ServerError> {
        let user_opt = sqlx::query!(
            "SELECT email, role as \"role: Role\" FROM Users WHERE email = ?",
            email
        )
        .fetch_optional(db())
        .await?;
        let active_sessions = Session::get_all_sessions_for_email(email).await?;

        let user_opt = user_opt.map(|user| User {
            email: user.email,
            role: user.role,
            active_sessions,
        });
        Ok(user_opt)
    }

    pub async fn get_from_uuid(uuid: &str) -> Result<Option<User>, ServerError> {
        let email_record = match sqlx::query!(
            "SELECT email FROM Users INNER JOIN Sessions ON Sessions.user_id = Users.id WHERE uuid = ?",
            uuid
        )
        .fetch_optional(db())
        .await?
        {
            Some(user_id) => user_id,
            None => return Ok(None),
        };
        User::get_from_email(&email_record.email).await
    }

    pub async fn update_role(self, new_role: Role) -> Result<(), ServerError> {
        sqlx::query!(
            "UPDATE Users SET role = ? WHERE email = ?",
            new_role,
            self.email
        )
        .execute(db())
        .await?;
        Ok(())
    }

    pub async fn delete(self) -> Result<(), ServerError> {
        sqlx::query!("DELETE FROM Users WHERE email = ?", self.email)
            .execute(db())
            .await
            .map_err(ServerError::Sqlx)?;
        Ok(())
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for User {
    type Rejection = UserParseError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let cookies = CookieJar::from_request_parts(parts, _state)
            .await
            .map_err(|_| UserParseError::CannotExtractCookies)?;
        let session_uuid = cookies
            .get("session")
            .ok_or_else(|| UserParseError::SessionNotFound)?
            .value();
        let user = User::get_from_uuid(session_uuid)
            .await?
            .ok_or_else(|| UserParseError::UserNotFound)?;
        Ok(user)
    }
}

pub struct AdminUser(pub User);
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for AdminUser {
    type Rejection = UserParseError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user = User::from_request_parts(parts, _state).await?;
        if let Role::Admin = user.role {
            Ok(AdminUser(user))
        } else {
            Err(UserParseError::NotAdmin(user.email))
        }
    }
}
