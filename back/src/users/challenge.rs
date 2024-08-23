use crate::{
    errors::{ChallengeVerifyError, CreateChallengeError},
    users::{mail::MailManager, session::Session, user::User},
};

use rand::{rngs::StdRng, Rng, SeedableRng};
use rocket::{
    http::{Cookie, CookieJar},
    serde::json::{json, Json, Value},
    State,
};
use sqlx::{types::time::OffsetDateTime, MySql, Pool};
use std::{collections::HashMap, time::Duration};
use tokio::sync::RwLock;

type Email = String;
pub struct ChallengeManager {
    challenges: RwLock<HashMap<Email, Challenge>>,
}
impl ChallengeManager {
    pub fn new() -> ChallengeManager {
        ChallengeManager {
            challenges: RwLock::new(HashMap::new()),
        }
    }
    pub async fn create_challenge(
        &self,
        pool: &Pool<MySql>,
        email: &str,
        mail_manager: &State<MailManager>,
    ) -> Result<(), CreateChallengeError> {
        let mut challenges = self.challenges.write().await;
        User::get_from_email(pool, email)
            .await?
            .ok_or_else(|| CreateChallengeError::UserNotFound(email.to_owned()))?;
        let challenge = Challenge::new();

        let code = challenge
            .code
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .chunks(2)
            .map(|chunk| chunk.concat())
            .collect::<Vec<String>>()
            .join(" - ");

        mail_manager.send_code(email.to_string(), code).await;
        //doesn't matter if the user already has a challenge, we want it to be overwritten (new attempt)
        challenges.insert(email.to_owned(), challenge);

        Ok(())
    }

    pub async fn verify_challenge(
        &self,
        pool: &Pool<MySql>,
        email: &str,
        user_code: &str,
    ) -> Result<Session, ChallengeVerifyError> {
        let challenges = self.challenges.read().await;
        let challenge = challenges
            .get(email)
            .ok_or(ChallengeVerifyError::UserNotFound(email.to_string()))?;

        if OffsetDateTime::now_utc() > challenge.expires {
            return Err(ChallengeVerifyError::ChallengeExpired(email.to_string()));
        }

        let user_code_u8 = user_code
            .chars()
            .filter_map(|e| {
                e.to_digit(10)
                    .map(|digit| if digit < 10 { Some(digit as u8) } else { None })
            })
            .collect::<Option<Vec<u8>>>()
            .ok_or(ChallengeVerifyError::InvalidCode(format!("{user_code:?}")))?;

        let user_code: [u8; 6] = user_code_u8
            .try_into()
            .map_err(|_| ChallengeVerifyError::InvalidCode(format!("{user_code:?}")))?;

        let email = email.to_string();
        let challenge_code = challenge.code;
        drop(challenges);
        if challenge_code == user_code {
            let mut challenges = self.challenges.write().await;
            challenges.remove(&email);
            let session = Session::new(pool, email).await?;
            Ok(session)
        } else {
            Err(ChallengeVerifyError::ChallengeFailed(email.to_string()))
        }
    }
}

const CHALLENGE_DURATION: Duration = Duration::from_secs(60 * 10);
struct Challenge {
    code: [u8; 6],
    expires: OffsetDateTime,
}
impl Challenge {
    fn new() -> Challenge {
        let code: [u8; 6] = (0..6)
            .map(|_| StdRng::from_entropy().gen_range(0..10) as u8)
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        println!("created challenge with code {code:?}");

        let expires = OffsetDateTime::now_utc() + CHALLENGE_DURATION;
        Challenge { code, expires }
    }
}

#[post("/create?<email>")]
pub async fn create_challenge(
    pool: &State<Pool<MySql>>,
    challenge_manager: &State<ChallengeManager>,
    mail_manager: &State<MailManager>,
    email: String,
) -> Result<Json<Value>, CreateChallengeError> {
    challenge_manager
        .create_challenge(pool, &email, mail_manager)
        .await?;
    Ok(Json(json!({"success": true})))
}

#[post("/verify?<email>&<code>")]
pub async fn verify_challenge(
    cookies: &CookieJar<'_>,
    challenge_manager: &State<ChallengeManager>,
    pool: &State<Pool<MySql>>,
    email: String,
    code: String,
) -> Result<Json<Value>, ChallengeVerifyError> {
    let session = challenge_manager
        .verify_challenge(pool, &email, &code)
        .await?;

    let cookie = Cookie::build(("session", session.uuid))
        .expires(session.expires)
        .secure(true);

    cookies.add(cookie);
    Ok(Json(json!({"success": true})))
}


