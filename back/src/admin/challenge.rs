use lettre::message::Mailbox;
use rand::{rngs::StdRng, Rng, SeedableRng};
use sqlx::types::time::OffsetDateTime;
use std::{collections::HashMap, time::Duration};
use tokio::sync::RwLock;

use crate::errors::SessionError;

use super::{auth::Session, mail, user::User};

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

        let expires = OffsetDateTime::now_utc() + CHALLENGE_DURATION;
        Challenge { code, expires }
    }
}

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
    pub async fn create_challenge(&self, email: &str) -> Result<(), SessionError> {
        let mut challenges = self.challenges.write().await;
        User::get_from_email(email)
            .await?
            .ok_or_else(|| SessionError::ChallengeNotFound(email.to_owned()))?;
        let challenge = Challenge::new();

        let email: Mailbox = email.parse()?;

        mail::send_code(&email, challenge.code).await?;

        //doesn't matter if the user already has a challenge, we want it to be overwritten (new attempt)
        challenges.insert(email.to_string(), challenge);

        Ok(())
    }

    pub async fn verify_challenge(
        &self,
        email: &str,
        user_code: &str,
    ) -> Result<Session, SessionError> {
        let challenges = self.challenges.read().await;
        let challenge = challenges
            .get(email)
            .ok_or(SessionError::ChallengeNotFound(email.to_string()))?;

        if OffsetDateTime::now_utc() > challenge.expires {
            return Err(SessionError::ChallengeExpired(email.to_string()));
        }

        let user_code_u8 = user_code
            .chars()
            .filter_map(|e| {
                e.to_digit(10)
                    .map(|digit| if digit < 10 { Some(digit as u8) } else { None })
            })
            .collect::<Option<Vec<u8>>>()
            .ok_or(SessionError::InvalidCode(format!("{user_code:?}")))?;

        let user_code: [u8; 6] = user_code_u8
            .try_into()
            .map_err(|_| SessionError::InvalidCode(format!("{user_code:?}")))?;

        let email = email.to_string();
        let challenge_code = challenge.code;
        drop(challenges);
        if challenge_code == user_code {
            let mut challenges = self.challenges.write().await;
            challenges.remove(&email);
            let session = Session::new(email).await?;
            Ok(session)
        } else {
            Err(SessionError::ChallengeFailed(email.to_string()))
        }
    }
}
