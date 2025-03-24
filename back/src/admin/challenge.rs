use lettre::{message::Mailbox, Message};
use rand::{rngs::StdRng, Rng, SeedableRng};
use sqlx::{types::time::OffsetDateTime, MySqlPool};
use std::{collections::HashMap, env, time::Duration};
use tokio::sync::RwLock;

use crate::errors::{ServerError, SessionError};

use super::user::User;

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
#[derive(Default)]
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
        pool: &MySqlPool,
        email: &str,
    ) -> Result<Message, SessionError> {
        let mut challenges = self.challenges.write().await;
        User::get_from_email(pool, email)
            .await?
            .ok_or_else(|| SessionError::ChallengeNotFound(email.to_owned()))?;
        let challenge = Challenge::new();

        let code = challenge
            .code
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .chunks(2)
            .map(|chunk| chunk.concat())
            .collect::<Vec<String>>();

        if cfg!(debug_assertions) {
            println!(
                "challenge created for user {email}, code is: {:?}",
                code.join(" - ")
            );
        }
        let from: Mailbox = env::var("SMTP_USERNAME")
            .expect("could not find app email in env")
            .parse()?;
        let to: Mailbox = email.parse()?;
        let login_link = env::var("VITE_SITE_URL")
            .map(|e| format!("{e}/login?email={to}&code={}", code.join("")))
            .unwrap_or("".to_string());
        let email = Message::builder()
        .from(from)
        .to(to.clone())
        .subject(format!("connexion à biere n collect pour le bar {}", env::var("VITE_BAR_NAME").unwrap_or("".into())))
        .header(lettre::message::header::ContentType::TEXT_PLAIN)
        .body(format!(
            "Une tentative de connexion pour le compte
            {to}
            a été détéctée.
            \n\nAfin de vous connecter, veuillez saisir le code \n{}\n dans l'invite.
            Vous pouvez également cliquer sur le lien ci dessous :
            {login_link}
            \n\n\n--------\n\n<i>Ignorez ce message si vous n'êtes pas à l'origine de la connexion.</i>",
            code.join(" - ")
        )).map_err(ServerError::EmailBuild)?;

        //doesn't matter if the user already has a challenge, we want it to be overwritten (new attempt)
        challenges.insert(to.to_string(), challenge);

        Ok(email)
    }

    pub async fn verify_challenge(
        &self,
        email: &str,
        user_code: &str,
    ) -> Result<bool, SessionError> {
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
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
