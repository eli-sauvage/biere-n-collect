use std::fmt::Display;

use crate::{
    errors::{ServerError, UserManagementError},
    users::session::Session,
};
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome},
    serde::json::{json, Json, Value},
    Request, State,
};
use serde::Serialize;
use sqlx::{MySql, Pool};

#[derive(Eq, PartialEq, Hash, Clone, Debug, Serialize)]
pub enum Role {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "waiter")]
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

#[derive(Clone, Debug, Serialize)]
pub struct User {
    pub email: String,
    pub role: Role,
    pub active_sessions: Vec<Session>,
}
impl User {
    pub async fn new(pool: &Pool<MySql>, email: String, role: Role) -> Result<User, ServerError> {
        sqlx::query!(
            "INSERT INTO Users (email, role) VALUES (?, ?)",
            email,
            role.to_string()
        )
        .execute(pool)
        .await
        .map_err(ServerError::Sqlx)?;
        Ok(User {
            email,
            role,
            active_sessions: vec![],
        })
    }

    pub async fn get_all(pool: &Pool<MySql>) -> Result<Vec<User>, ServerError> {
        let record = sqlx::query!("SELECT email, role FROM Users")
            .fetch_all(pool)
            .await?;
        let all_sessions = Session::get_all(pool).await?;
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
                    role: r.role.into(),
                    active_sessions: sessions,
                }
            })
            .collect();
        Ok(users)
    }

    pub async fn get_from_email(
        pool: &Pool<MySql>,
        email: &str,
    ) -> Result<Option<User>, ServerError> {
        let user_opt = sqlx::query!("SELECT email, role FROM Users WHERE email = ?", email)
            .fetch_optional(pool)
            .await?;
        let active_sessions = Session::get_all_sessions_for_email(pool, &email).await?;

        let user_opt = user_opt.map(|user| User {
            email: user.email,
            role: user.role.into(),
            active_sessions,
        });
        Ok(user_opt)
    }

    pub async fn get_from_uuid(
        pool: &Pool<MySql>,
        uuid: String,
    ) -> Result<Option<User>, ServerError> {
        let email_record = match sqlx::query!(
            "SELECT email FROM Users INNER JOIN Sessions ON Sessions.user_id = Users.id WHERE uuid = ?",
            uuid
        )
        .fetch_optional(pool)
        .await?
        {
            Some(user_id) => user_id,
            None => return Ok(None),
        };
        User::get_from_email(pool, &email_record.email).await
    }

    pub async fn update_role(self, pool: &Pool<MySql>, new_role: Role) -> Result<(), ServerError> {
        sqlx::query!(
            "UPDATE Users SET role = ? WHERE email = ?",
            new_role.to_string(),
            self.email
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(self, pool: &Pool<MySql>) -> Result<(), ServerError> {
        sqlx::query!("DELETE FROM Users WHERE email = ?", self.email)
            .execute(pool)
            .await
            .map_err(ServerError::Sqlx)?;
        Ok(())
    }
}

pub type ErrorMsg = String;
#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ErrorMsg;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let invalid_session = || "invalid session".to_string();
        let uuid: String = match req.cookies().get("session") {
            Some(uuid) => uuid.value().to_owned(),
            None => return Outcome::Error((Status::Unauthorized, invalid_session())),
        };

        let server_error = || "server error during authentication".to_string();
        let pool = match req.rocket().state::<Pool<MySql>>() {
            Some(pool) => pool,
            None => return Outcome::Error((Status::InternalServerError, server_error())),
        };

        let user_opt = match User::get_from_uuid(pool, uuid).await {
            Ok(user) => user,
            Err(_) => return Outcome::Error((Status::InternalServerError, server_error())),
        };

        let user = match user_opt {
            Some(user) => user,
            None => return Outcome::Error((Status::Unauthorized, invalid_session())),
        };

        Outcome::Success(user)
    }
}

pub struct AdminUser(User);
#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = ErrorMsg;
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match User::from_request(req).await {
            Outcome::Success(user) => {
                if user.role == Role::Admin {
                    Outcome::Success(AdminUser(user))
                } else {
                    Outcome::Error((
                        Status::Unauthorized,
                        format!("user {} is not admin", user.email),
                    ))
                }
            }
            Outcome::Forward(f) => Outcome::Forward(f),
            Outcome::Error(e) => Outcome::Error(e),
        }
    }
}

#[get("/get_all")]
pub async fn get_all_users(
    user: Result<AdminUser, ErrorMsg>,
    pool: &State<Pool<MySql>>,
) -> Result<Json<Vec<Value>>, UserManagementError> {
    if let Err(e) = user {
        return Err(UserManagementError::NotAdmin(e));
    }
    let all_users = User::get_all(pool).await?;
    let res: Vec<Value> = all_users
        .into_iter()
        .map(|u| {
            json!({
                "email": u.email,
                "role": u.role,
                "sessions": u.active_sessions.len()
            })
        })
        .collect();

    Ok(Json(res))
}

#[delete("/?<email>")]
pub async fn delete_user(
    user: Result<AdminUser, ErrorMsg>,
    pool: &State<Pool<MySql>>,
    email: String,
) -> Result<Json<Value>, UserManagementError> {
    let user = match user {
        Ok(u) => u,
        Err(e) => return Err(UserManagementError::NotAdmin(e)),
    };
    if user.0.email == email {
        return Err(UserManagementError::UserCannotUpdateItSelf);
    }

    let user_to_delete = User::get_from_email(pool, &email)
        .await?
        .ok_or_else(|| UserManagementError::UserDoesNotExist(email))?;

    for session in &user_to_delete.active_sessions {
        Session::delete_if_exists(pool, &session.uuid).await?;
    }

    user_to_delete.delete(pool).await?;

    Ok(Json(json!({"success": true})))
}

#[patch("/update_role?<email>&<new_role>")]
pub async fn update_role(
    user: Result<AdminUser, ErrorMsg>,
    pool: &State<Pool<MySql>>,
    email: String,
    new_role: String,
) -> Result<Json<Value>, UserManagementError> {
    let user = match user {
        Ok(u) => u,
        Err(e) => return Err(UserManagementError::NotAdmin(e)),
    };
    if user.0.email == email {
        return Err(UserManagementError::UserCannotUpdateItSelf);
    }

    let user_to_update = User::get_from_email(pool, &email)
        .await?
        .ok_or_else(|| UserManagementError::UserDoesNotExist(email))?;

    let new_role = match new_role.as_str() {
        "admin" => Role::Admin,
        "waiter" => Role::Waiter,
        _ => return Err(UserManagementError::RoleDoesNotExist(new_role)),
    };

    user_to_update.update_role(pool, new_role).await?;

    Ok(Json(json!({"success": true})))
}

#[post("/?<email>&<role>")]
pub async fn add_user(
    user: Result<AdminUser, ErrorMsg>,
    pool: &State<Pool<MySql>>,
    email: String,
    role: String,
) -> Result<Json<Value>, UserManagementError> {
    if let Err(e) = user {
        return Err(UserManagementError::NotAdmin(e));
    }

    let new_role = match role.as_str() {
        "admin" => Role::Admin,
        "waiter" => Role::Waiter,
        _ => return Err(UserManagementError::RoleDoesNotExist(role)),
    };

    User::new(pool, email, new_role).await?;

    Ok(Json(json!({"success": true})))
}

#[patch("/disconnect?<email>")]
pub async fn disconnect_user(
    user: Result<AdminUser, ErrorMsg>,
    pool: &State<Pool<MySql>>,
    email: String,
) -> Result<Json<Value>, UserManagementError> {
    let user = match user {
        Ok(u) => u,
        Err(e) => return Err(UserManagementError::NotAdmin(e)),
    };
    if user.0.email == email {
        return Err(UserManagementError::UserCannotUpdateItSelf);
    }

    let user_to_disconnect = User::get_from_email(pool, &email)
        .await?
        .ok_or_else(|| UserManagementError::UserDoesNotExist(email))?;

    for session in &user_to_disconnect.active_sessions {
        Session::delete_if_exists(pool, &session.uuid).await?;
    }

    Ok(Json(json!({"success": true})))
}
