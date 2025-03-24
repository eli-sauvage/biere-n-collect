use crate::errors::{ServerError, SessionError};

use sqlx::{types::time::OffsetDateTime, SqlitePool};
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
    async fn delete_old_sessions(pool: &SqlitePool) -> Result<(), ServerError> {
        sqlx::query!("DELETE FROM Sessions WHERE CURRENT_TIMESTAMP > expires")
            .execute(pool)
            .await
            .map_err(ServerError::Sqlx)?;
        Ok(())
    }

    pub async fn delete_if_exists(pool: &SqlitePool, uuid: &str) -> Result<(), ServerError> {
        sqlx::query!("DELETE FROM Sessions WHERE uuid = ?", uuid)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn new(pool: &SqlitePool, email: String) -> Result<Session, SessionError> {
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

    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Session>, ServerError> {
        let sessions = sqlx::query_as!(Session, "SELECT email, expires, uuid FROM Sessions INNER JOIN Users ON Users.id = Sessions.user_id")
            .fetch_all(pool)
            .await?;

        Ok(sessions)
    }

    pub async fn get_all_sessions_for_email(
        pool: &SqlitePool,
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

#[sqlx::test]
async fn test_new_session(pool: SqlitePool) {
    let email = "elicolh@gmail.com";
    let res = Session::new(&pool, email.into()).await.unwrap();
    assert_eq!(res.email, email);
    //valid uuid
    let session_uuid = <uuid::Uuid as std::str::FromStr>::from_str(&res.uuid).unwrap();
    assert_eq!(session_uuid.get_version(), Some(uuid::Version::Random));

    let user = User::get_from_email(&pool, email).await.unwrap().unwrap();
    let session_in_db = sqlx::query!("SELECT * FROM Sessions WHERE uuid = ?", res.uuid)
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(session_in_db.user_id as u32, user.id);
    assert_eq!(session_in_db.uuid, res.uuid);
    assert_eq!(
        session_in_db.expires.replace_millisecond(0).unwrap(),
        res.expires.replace_millisecond(0).unwrap() //sub second is kind of random ?
    );
}

#[sqlx::test]
async fn test_new_session_for_non_existant_user(pool: SqlitePool) {
    let res = Session::new(&pool, "test@example.com".into()).await;
    assert!(res.is_err());
    if let Err(SessionError::UserNotFound(email)) = res {
        assert_eq!(email, "test@example.com")
    } else {
        panic!("error should be UserNotFound")
    }
}

#[sqlx::test]
async fn test_get_all(pool: SqlitePool) {
    let email = "elicolh@gmail.com";
    let session1 = Session::new(&pool, email.to_owned()).await.unwrap();
    let session2 = Session::new(&pool, email.to_owned()).await.unwrap();

    let sessions = Session::get_all(&pool).await.unwrap();
    assert_eq!(sessions.len(), 2);
    assert_eq!(sessions[0].uuid, session1.uuid);
    assert_eq!(sessions[1].uuid, session2.uuid);
}

#[sqlx::test]
async fn test_get_all_no_sessions(pool: SqlitePool) {
    let all_sessions = Session::get_all(&pool).await.unwrap();
    assert!(all_sessions.is_empty());
}

#[sqlx::test]
async fn test_get_all_for_email(pool: SqlitePool) {
    let email1 = "elicolh@gmail.com";
    let email2 = "eli.sauvage@utt.fr";
    let session1 = Session::new(&pool, email1.to_owned()).await.unwrap();
    Session::new(&pool, email2.to_owned()).await.unwrap();

    let sessions = Session::get_all_sessions_for_email(&pool, email1)
        .await
        .unwrap();
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].uuid, session1.uuid);
}

#[sqlx::test]
async fn test_get_all_for_email_no_session(pool: SqlitePool) {
    let email1 = "elicolh@gmail.com";
    let email2 = "eli.sauvage@utt.fr";
    Session::new(&pool, email2.to_owned()).await.unwrap();

    let sessions = Session::get_all_sessions_for_email(&pool, email1)
        .await
        .unwrap();

    assert!(sessions.is_empty());
}

#[sqlx::test]
async fn delete_old_sessions_test(pool: SqlitePool) {
    let email = "elicolh@gmail.com";
    let session1 = Session::new(&pool, email.to_owned()).await.unwrap();
    let session2 = Session::new(&pool, email.to_owned()).await.unwrap();

    sqlx::query!(
        "UPDATE Sessions SET expires = datetime(CURRENT_TIMESTAMP, '-1 minute') WHERE uuid = ?",
        session1.uuid
    )
    .execute(&pool)
    .await
    .unwrap();

    Session::delete_old_sessions(&pool).await.unwrap();
    let sessions = Session::get_all(&pool).await.unwrap();
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].uuid, session2.uuid);
}

#[sqlx::test]
async fn test_delete_if_exists(pool: SqlitePool) {
    let email = "elicolh@gmail.com";
    let session1 = Session::new(&pool, email.to_owned()).await.unwrap();
    let session2 = Session::new(&pool, email.to_owned()).await.unwrap();

    Session::delete_if_exists(&pool, &session1.uuid)
        .await
        .unwrap();
    let sessions = Session::get_all(&pool).await.unwrap();
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].uuid, session2.uuid);
}
