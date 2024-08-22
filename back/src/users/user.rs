use std::fmt::Display;

use crate::{errors::ServerError, users::session::Session};
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome},
    Request,
};
use sqlx::{MySql, Pool};
use uuid::Uuid;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Role {
    Admin,
    Waiter,
}
impl From<String> for Role {
    fn from(value: String) -> Self {
        match value.as_str() {
            "admin" => Self::Admin,
            "waiter" => Self::Waiter,
            _ => panic!("received a role that is not admin neither waiter"),
        }
    }
}
impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "admin"),
            Role::Waiter => write!(f, "waiter"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct User {
    pub email: String,
    pub role: Role,
    pub session: Option<Session>,
}
impl User {
    pub async fn get_from_email(
        pool: &Pool<MySql>,
        email: String,
    ) -> Result<Option<User>, ServerError> {
        let user_opt = sqlx::query!("SELECT email, role FROM Users WHERE email = ?", email)
            .fetch_optional(pool)
            .await
            .map_err(ServerError::Sqlx)?;
        let session = if user_opt.is_some() {
            Session::get_from_email(pool, &email).await?
        } else {
            None
        };

        Ok(user_opt.map(|user| User {
            email: user.email,
            role: user.role.into(),
            session,
        }))
    }

    pub async fn get_from_uuid(
        pool: &Pool<MySql>,
        uuid: Uuid,
    ) -> Result<Option<User>, ServerError> {
        let email_record = match sqlx::query!(
            "SELECT email FROM Users INNER JOIN Sessions ON Sessions.user_id = Users.id WHERE uuid = ?",
            uuid.to_string()
        )
        .fetch_optional(pool)
        .await?
        {
            Some(user_id) => user_id,
            None => return Ok(None),
        };
        User::get_from_email(pool, email_record.email).await
    }

    pub async fn add_to_db(&self, pool: &Pool<MySql>) -> Result<(), ServerError> {
        sqlx::query!(
            "INSERT INTO Users (email, role) VALUES (?, ?)",
            self.email,
            self.role.to_string()
        )
        .execute(pool)
        .await
        .map_err(ServerError::Sqlx)?;
        Ok(())
    }

    pub async fn remove_from_db(self, pool: &Pool<MySql>) -> Result<(), ServerError> {
        sqlx::query!("DELETE FROM Users WHERE email = ?", self.email)
            .execute(pool)
            .await
            .map_err(ServerError::Sqlx)?;
        Ok(())
    }
}

pub type UserGuardError = ();
#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = UserGuardError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let uuid: String = match req.cookies().get("session") {
            Some(uuid) => uuid.value().to_owned(),
            None => return Outcome::Error((Status::Unauthorized, ())),
        };
        let uuid: Uuid = match uuid.parse() {
            Ok(uuid) => uuid,
            Err(_) => return Outcome::Error((Status::Unauthorized, ())),
        };

        let pool = match req.rocket().state::<Pool<MySql>>() {
            Some(pool) => pool,
            None => return Outcome::Error((Status::InternalServerError, ())),
        };

        let user_opt = match User::get_from_uuid(pool, uuid).await {
            Ok(user) => user,
            Err(_) => return Outcome::Error((Status::InternalServerError, ())),
        };

        let user = match user_opt {
            Some(user) => user,
            None => return Outcome::Error((Status::Unauthorized, ())),
        };

        Outcome::Success(user)
    }
}
