use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize, Serializer};
use sqlx::MySqlPool;

use crate::{
    errors::{ServerError, UserManagementError, UserParseError},
    routes::AppState,
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
    #[serde(skip)]
    pub id: u32,
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
    pub async fn create(
        pool: &MySqlPool,
        email: &str,
        role: Role,
    ) -> Result<User, UserManagementError> {
        let existing_user = User::get_from_email(pool, email).await?;
        if existing_user.is_some() {
            return Err(UserManagementError::UserAlreadyExists(email.to_owned()));
        }
        let id = sqlx::query!("INSERT INTO Users (email, role) VALUES (?, ?)", email, role)
            .execute(pool)
            .await
            .map_err(ServerError::Sqlx)?
            .last_insert_id() as u32;
        Ok(User {
            id,
            email: email.to_owned(),
            role,
            active_sessions: vec![],
        })
    }

    pub async fn get_all(pool: &MySqlPool) -> Result<Vec<User>, ServerError> {
        let record = sqlx::query!("SELECT id, email, role as \"role: Role\" FROM Users")
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
                    id: r.id,
                    email: r.email,
                    role: r.role,
                    active_sessions: sessions,
                }
            })
            .collect();
        Ok(users)
    }

    pub async fn get_from_email(
        pool: &MySqlPool,
        email: &str,
    ) -> Result<Option<User>, ServerError> {
        let user_opt = sqlx::query!(
            "SELECT id, email, role as \"role: Role\" FROM Users WHERE email = ?",
            email
        )
        .fetch_optional(pool)
        .await?;
        let active_sessions = Session::get_all_sessions_for_email(pool, email).await?;

        let user_opt = user_opt.map(|user| User {
            id: user.id,
            email: user.email,
            role: user.role,
            active_sessions,
        });
        Ok(user_opt)
    }

    pub async fn get_from_uuid(pool: &MySqlPool, uuid: &str) -> Result<Option<User>, ServerError> {
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

    pub async fn update_role(self, pool: &MySqlPool, new_role: Role) -> Result<(), ServerError> {
        sqlx::query!(
            "UPDATE Users SET role = ? WHERE email = ?",
            new_role,
            self.email
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(self, pool: &MySqlPool) -> Result<(), ServerError> {
        sqlx::query!("DELETE FROM Users WHERE email = ?", self.email)
            .execute(pool)
            .await
            .map_err(ServerError::Sqlx)?;
        Ok(())
    }
}

#[async_trait]
impl FromRequestParts<AppState> for User {
    type Rejection = UserParseError;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let cookies = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| UserParseError::CannotExtractCookies)?;
        let session_uuid = cookies
            .get("session")
            .ok_or_else(|| UserParseError::SessionNotFound)?
            .value();

        let user = User::get_from_uuid(&state.pool, session_uuid)
            .await?
            .ok_or_else(|| UserParseError::UserNotFound)?;
        Ok(user)
    }
}

pub struct AdminUser(pub User);
#[async_trait]
impl FromRequestParts<AppState> for AdminUser {
    type Rejection = UserParseError;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let user = User::from_request_parts(parts, state).await?;
        if let Role::Admin = user.role {
            Ok(AdminUser(user))
        } else {
            Err(UserParseError::NotAdmin(user.email))
        }
    }
}

#[sqlx::test]
async fn test_user_create_update_role_delete(pool: MySqlPool) {
    let email = "user@example.com";
    let role = Role::Admin;
    let user = User::create(&pool, email, role).await.unwrap();
    assert_eq!(user.email, email);
    assert!(user.active_sessions.is_empty());
    assert_eq!(user.role, Role::Admin);

    user.update_role(&pool, Role::Waiter).await.unwrap();
    let user = User::get_from_email(&pool, email).await.unwrap().unwrap();
    assert_eq!(user.role, Role::Waiter);

    user.delete(&pool).await.unwrap();
    let user = User::get_from_email(&pool, email).await.unwrap();
    assert!(user.is_none());
}

#[sqlx::test]
async fn test_user_duplicate(pool: MySqlPool) {
    let email = "user@example.com";
    let role = Role::Admin;
    User::create(&pool, email, role).await.unwrap();
    let user2 = User::create(&pool, email, role).await;
    assert!(user2.is_err());
    match user2.unwrap_err() {
        UserManagementError::UserAlreadyExists(m) => {
            assert_eq!(m, email);
        }
        _ => panic!("wrong error on duplicate user"),
    };
}

#[sqlx::test]
async fn test_user_extractor(pool: MySqlPool) {
    use axum::{
        http::{method::Method, Request},
        routing::get,
        Router,
    };
    let email = "user@example.com";
    let user = User::create(&pool, email, Role::Waiter).await.unwrap();
    let session = Session::new(&pool, email.to_owned()).await.unwrap();
    let app = Router::new().route("/", get(test_fn));
    async fn test_fn(user: User) {
        assert_eq!(user.email, "user@example.com");
    }

    let request = Request::builder()
        .method(Method::GET)
        .uri("/")
        .header("Cookie", format!("session={}", session.uuid))
        .body("")
        .unwrap();


}
