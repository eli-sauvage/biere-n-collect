pub(crate) mod admin;
pub(crate) mod customer;

pub(crate) mod cors;
pub(crate) mod extractors;
pub(crate) mod reponders;

use crate::admin::challenge::ChallengeManager;
use std::sync::Arc;

pub struct InnerState {
    pub challenge_manager: ChallengeManager,
}
pub type AppState = Arc<InnerState>;

pub fn generate_app_state(challenge_manager: ChallengeManager) -> AppState {
    Arc::new(InnerState { challenge_manager })
}
