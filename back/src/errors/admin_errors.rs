use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

use super::ErrorResponse;

#[derive(Error, Debug)]
pub enum UserManagementError {
    #[error("user already exists with email {0}")]
    UserAlreadyExists(String),
    #[error("invalid email address {1}: {0}")]
    InvalidEmailAddress(String, lettre::address::AddressError),
    #[error("user could not be identified")]
    UserDoesNotExist(String),
    #[error("A user cannot modify its role or delete itself")]
    UserCannotUpdateItSelf,
    #[error("server error")]
    ServerError(#[from] crate::errors::ServerError),
}
impl IntoResponse for UserManagementError {
    fn into_response(self) -> axum::response::Response {
        if let UserManagementError::ServerError(e) = self {
            e.into_response()
        } else {
            let status = match self {
                Self::UserDoesNotExist(_) => StatusCode::NOT_FOUND,
                UserManagementError::UserCannotUpdateItSelf
                | UserManagementError::UserAlreadyExists(_)
                | UserManagementError::InvalidEmailAddress(_, _) => StatusCode::BAD_REQUEST,
                Self::ServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (status, ErrorResponse::json(self.to_string())).into_response()
        }
    }
}

#[derive(Error, Debug)]
pub enum SessionError {
    #[error("user {0} does not have a challenge")]
    ChallengeNotFound(String),
    #[error("session not found for current user")]
    SessionNotFound,
    #[error("challenge for user {0} has expired")]
    ChallengeExpired(String),
    #[error("the code submitted for user {0} is invalid")]
    ChallengeFailed(String),
    #[error("the code {0} is invalid, expected : 6 digits")]
    InvalidCode(String),
    #[error("invalid email address: {0}")]
    InvalidEmailAddress(#[from] lettre::address::AddressError),
    #[error("server error")]
    ServerError(#[from] crate::errors::ServerError),
}
impl IntoResponse for SessionError {
    fn into_response(self) -> axum::response::Response {
        if let SessionError::ServerError(e) = self {
            e.into_response()
        } else {
            let status = match &self {
                Self::ChallengeNotFound(_)
                | Self::ChallengeExpired(_)
                | Self::ChallengeFailed(_)
                | Self::SessionNotFound
                | Self::InvalidCode(_)
                | Self::InvalidEmailAddress(_) => StatusCode::BAD_REQUEST,
                Self::ServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (status, ErrorResponse::json(self.to_string())).into_response()
        }
    }
}

#[derive(Error, Debug)]
pub enum UserParseError {
    #[error("could not extract cookies")]
    CannotExtractCookies,
    #[error("session not found")]
    SessionNotFound,
    #[error("user not found in db")]
    UserNotFound,
    #[error("the user {0} does not have an Admin role")]
    NotAdmin(String),
    #[error("server error")]
    ServerError(#[from] crate::errors::ServerError),
}
impl IntoResponse for UserParseError {
    fn into_response(self) -> axum::response::Response {
        if let UserParseError::ServerError(e) = self {
            e.into_response()
        } else {
            let status = match self {
                Self::CannotExtractCookies
                | Self::SessionNotFound
                | Self::UserNotFound
                | Self::NotAdmin(_) => StatusCode::UNAUTHORIZED,
                Self::ServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };
            (status, ErrorResponse::json(self.to_string())).into_response()
        }
    }
}
