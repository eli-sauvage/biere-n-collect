pub(crate) mod admin;
pub(crate) mod customer;

pub(crate) mod cors;
pub(crate) mod extractors;
pub(crate) mod reponders;

use sqlx::MySqlPool;

use crate::{admin::challenge::ChallengeManager, mail_manager::MailManager};
use std::sync::Arc;

pub struct InnerState {
    pub challenge_manager: ChallengeManager,
    pub pool: MySqlPool,
    pub mail_manager: Box<dyn MailManager>,
}
pub type AppState = Arc<InnerState>;

pub fn generate_app_state(
    challenge_manager: ChallengeManager,
    pool: MySqlPool,
    mail_manager: Box<dyn MailManager>,
) -> AppState {
    Arc::new(InnerState {
        challenge_manager,
        pool,
        mail_manager,
    })
}
