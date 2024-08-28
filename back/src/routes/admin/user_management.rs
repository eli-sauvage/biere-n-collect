use axum::{
    routing::{get, patch, post},
    Json, Router,
};
use serde::Deserialize;

use crate::{
    admin::{
        auth::Session,
        user::{AdminUser, Role, User},
    },
    errors::UserManagementError,
    routes::{AppState, CustomQuery as Query},
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
) -> Result<(), UserManagementError> {
    User::create(&params.email, params.role).await?;

    Ok(())
}

#[derive(Deserialize)]
struct DeleteUserParams {
    email: String,
}
async fn delete_user(
    user: AdminUser,
    params: Query<DeleteUserParams>,
) -> Result<(), UserManagementError> {
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

    Ok(())
}

#[derive(Deserialize)]
struct UpdateRoleParams {
    email: String,
    new_role: Role,
}
async fn update_role(
    user: AdminUser,
    params: Query<UpdateRoleParams>,
) -> Result<(), UserManagementError> {
    if user.0.email == params.email {
        return Err(UserManagementError::UserCannotUpdateItSelf);
    }

    let user_to_update = User::get_from_email(&params.email)
        .await?
        .ok_or_else(|| UserManagementError::UserDoesNotExist(params.email.clone()))?;

    user_to_update.update_role(params.new_role).await?;

    Ok(())
}

#[derive(Deserialize)]
struct DisconnectUserParams {
    email: String,
}
async fn disconnect_user(
    user: AdminUser,
    params: Query<DisconnectUserParams>,
) -> Result<(), UserManagementError> {
    if user.0.email == params.email {
        return Err(UserManagementError::UserCannotUpdateItSelf);
    }

    let user_to_disconnect = User::get_from_email(&params.email)
        .await?
        .ok_or_else(|| UserManagementError::UserDoesNotExist(params.email.clone()))?;

    for session in &user_to_disconnect.active_sessions {
        Session::delete_if_exists(&session.uuid).await?;
    }

    Ok(())
}
