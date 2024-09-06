use std::str::FromStr;

use axum::{
    routing::{get, patch, post},
    Json, Router,
};
use lettre::message::Mailbox;
use serde::Deserialize;

use crate::{
    admin::{
        auth::Session,
        user::{AdminUser, Role, User},
    },
    errors::UserManagementError,
    routes::{extractors::CustomQuery as Query, reponders::OkEmptyResponse, AppState},
};

pub fn get_router() -> Router<AppState> {
    Router::new()
        .route("/get_all", get(get_all_users))
        .route("/", post(add_user).delete(delete_user))
        .route("/update_role", patch(update_role))
        .route("/disconnect", patch(disconnect_user))
}

async fn get_all_users(_user: AdminUser) -> Result<Json<Vec<User>>, UserManagementError> {
    let all_users = User::get_all().await?;
    Ok(Json(all_users))
}

#[derive(Deserialize)]
struct AddUserParams {
    email: String,
    role: Role,
}
async fn add_user(
    _user: AdminUser,
    params: Query<AddUserParams>,
) -> Result<OkEmptyResponse, UserManagementError> {
    if let Some(_existing_user) = User::get_from_email(&params.email).await? {
        return Err(UserManagementError::UserAlreadyExists(params.email.clone()));
    }
    if let Err(e) = Mailbox::from_str(&params.email) {
        return Err(UserManagementError::InvalidEmailAddress(
            params.email.clone(),
            e,
        ));
    }
    User::create(&params.email, params.role).await?;

    Ok(OkEmptyResponse::new())
}

#[derive(Deserialize)]
struct DeleteUserParams {
    email: String,
}
async fn delete_user(
    user: AdminUser,
    params: Query<DeleteUserParams>,
) -> Result<OkEmptyResponse, UserManagementError> {
    if user.0.email == params.email {
        return Err(UserManagementError::UserCannotUpdateItSelf);
    }

    let user_to_delete = User::get_from_email(&params.email)
        .await?
        .ok_or_else(|| UserManagementError::UserDoesNotExist(params.email.clone()))?;

    for session in &user_to_delete.active_sessions {
        Session::delete_if_exists(&session.uuid).await?;
    }

    user_to_delete.delete().await?;

    Ok(OkEmptyResponse::new())
}

#[derive(Deserialize)]
struct UpdateRoleParams {
    email: String,
    new_role: Role,
}
async fn update_role(
    user: AdminUser,
    params: Query<UpdateRoleParams>,
) -> Result<OkEmptyResponse, UserManagementError> {
    if user.0.email == params.email {
        return Err(UserManagementError::UserCannotUpdateItSelf);
    }

    let user_to_update = User::get_from_email(&params.email)
        .await?
        .ok_or_else(|| UserManagementError::UserDoesNotExist(params.email.clone()))?;

    user_to_update.update_role(params.new_role).await?;

    Ok(OkEmptyResponse::new())
}

#[derive(Deserialize)]
struct DisconnectUserParams {
    email: String,
}
async fn disconnect_user(
    user: AdminUser,
    params: Query<DisconnectUserParams>,
) -> Result<OkEmptyResponse, UserManagementError> {
    if user.0.email == params.email {
        return Err(UserManagementError::UserCannotUpdateItSelf);
    }

    let user_to_disconnect = User::get_from_email(&params.email)
        .await?
        .ok_or_else(|| UserManagementError::UserDoesNotExist(params.email.clone()))?;

    for session in &user_to_disconnect.active_sessions {
        Session::delete_if_exists(&session.uuid).await?;
    }

    Ok(OkEmptyResponse::new())
}
