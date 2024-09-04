use std::time::Duration;

use reqwest::dns::Resolve;
use tokio::sync::{OnceCell, RwLock};

use crate::{db, errors::ServerError};

static CONFIG: OnceCell<RwLock<Config>> = OnceCell::const_new();
pub fn config() -> &'static RwLock<Config> {
    CONFIG.get().expect("global config is not initialized")
}

#[derive(Debug)]
pub struct Config {
    max_order_age: Duration,
    session_duration: Duration,
    stripe_secret_key: String,
    stripe_publishable_key: String,
    smtp_username: String,
    smtp_password: String,
}

impl Config {
    pub async fn init() -> Result<(), ServerError> {
        let row = sqlx::query!("SELECT * FROM Config").fetch_one(db()).await?;
        let config = Config {
            max_order_age: Duration::from_secs(row.max_order_age as u64),
            session_duration: Duration::from_secs(row.session_duration as u64),
            stripe_secret_key: row.stripe_secret_key,
            stripe_publishable_key: row.stripe_publishable_key,
            smtp_username: row.smtp_username,
            smtp_password: row.smtp_password,
        };
        CONFIG
            .set(RwLock::new(config))
            .expect("could not initialize static config");
        Ok(())
    }
    pub fn max_order_age(&self) -> &Duration {
        &self.max_order_age
    }
    pub async fn set_max_order_age(&mut self, new_duration: Duration) -> Result<(), ServerError> {
        self.max_order_age = new_duration;
        sqlx::query!(
            "UPDATE Config SET max_order_age = ?",
            new_duration.as_secs()
        )
        .execute(db())
        .await?;
        Ok(())
    }

    pub fn session_duration(&self) -> &Duration {
        &self.session_duration
    }
    pub fn session_duration_mut(&mut self) -> &mut Duration {
        &mut self.session_duration
    }

    pub fn stripe_secret_key(&self) -> &str {
        &self.stripe_secret_key
    }
    pub fn stripe_secret_key_mut(&mut self) -> &mut str {
        &mut self.stripe_secret_key
    }

    pub fn stripe_publishable_key(&self) -> &str {
        &self.stripe_publishable_key
    }
    pub fn stripe_publishable_key_mut(&mut self) -> &mut str {
        &mut self.stripe_publishable_key
    }

    pub fn smtp_username(&self) -> &str {
        &self.smtp_username
    }
    pub fn smtp_username_mut(&mut self) -> &mut str {
        &mut self.smtp_username
    }

    pub fn smtp_password(&self) -> &str {
        &self.smtp_password
    }
    pub fn smtp_password_mut(&mut self) -> &mut str {
        &mut self.smtp_password
    }
}
