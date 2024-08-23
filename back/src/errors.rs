use rocket::{
    http::Status,
    response::{self, Responder},
    serde::json::{json, Json},
    Response,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("sqlx error")]
    Sqlx(#[from] sqlx::error::Error),
    #[error("Migration error")]
    SqlxMigrate(#[from] sqlx::migrate::MigrateError),
    #[error("rocket error")]
    Rocket(#[from] rocket::Error),
    #[error("uuid error")]
    Uuid(#[from] uuid::Error),
    #[error("reqwest error")]
    Reqwest(#[from] reqwest::Error),
}

impl<'r> Responder<'r, 'static> for ServerError {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        eprintln!("{self:?}");
        let res = Json(json!({"error": "server error"})).respond_to(request)?;
        Response::build_from(res)
            .status(Status::InternalServerError)
            .ok()
    }
}

#[derive(Error, Debug)]
pub enum ManageStockError {
    #[error("user could not be verified as admin: {0}")]
    NotAdmin(String),
    #[error("le stock avec l'id {0} n'existe pas")]
    StockNotFound(u32),
    #[error("la direction {0} n'existe pas (\"up\" ou \"down\" uniquement)")]
    DirectionDoesNotExist(String),
    #[error("stock with id {0} cannot move up")]
    CannotMoveUp(u32),
    #[error("stock with id {0} cannot move down")]
    CannotMoveDown(u32),
    #[error("server error")]
    ServerError(#[from] ServerError),
}

impl<'r> Responder<'r, 'static> for ManageStockError {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> response::Result<'static> {
        let status = match &self {
            Self::NotAdmin(_) => Status::Unauthorized,
            Self::StockNotFound(_) => Status::NotFound,
            Self::DirectionDoesNotExist(_) | Self::CannotMoveUp(_) | Self::CannotMoveDown(_) => {
                Status::BadRequest
            }
            Self::ServerError(e) => {
                eprintln!("{e:?}");
                Status::InternalServerError
            }
        };
        let json = Json(json! ({"error": self.to_string()})).respond_to(request)?;
        Response::build_from(json).status(status).ok()
    }
}

#[derive(Error, Debug)]
pub enum OrderProcessError {
    #[error("pas assez de stock pour l'item {0}<#{1}>")]
    NotEnoughStock(String, u32),
    #[error("prouct not found (id = {0})")]
    ProductNotFound(u32),
    #[error("server error")]
    ServerError(#[from] ServerError),
}

impl<'r> Responder<'r, 'static> for OrderProcessError {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> response::Result<'static> {
        let status = match &self {
            Self::NotEnoughStock(_, _) | Self::ProductNotFound(_) => Status::BadRequest,
            Self::ServerError(e) => {
                eprintln!("{e:?}");
                Status::InternalServerError
            }
        };
        let json = Json(json! ({"error": self.to_string()})).respond_to(request)?;
        Response::build_from(json).status(status).ok()
    }
}

#[derive(Error, Debug)]
pub enum ChallengeVerifyError {
    #[error("user {0} does not have a challenge")]
    UserNotFound(String),
    #[error("challenge for user {0} has expired")]
    ChallengeExpired(String),
    #[error("the code submitted for user {0} is invalid")]
    ChallengeFailed(String),
    #[error("the code {0} is invalid, expected : 6 digits")]
    InvalidCode(String),
    #[error("server error")]
    ServerError(#[from] crate::errors::ServerError),
}
impl<'r> Responder<'r, 'static> for ChallengeVerifyError {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> response::Result<'static> {
        let status = match &self {
            Self::UserNotFound(_)
            | Self::ChallengeExpired(_)
            | Self::ChallengeFailed(_)
            | Self::InvalidCode(_) => Status::Unauthorized,
            Self::ServerError(e) => {
                eprintln!("{e:?}");
                Status::InternalServerError
            }
        };
        let json = Json(json! ({"error": self.to_string()})).respond_to(request)?;
        Response::build_from(json).status(status).ok()
    }
}

#[derive(Error, Debug)]
pub enum CreateChallengeError {
    #[error("the user {0} does not exist")]
    UserNotFound(String),
    #[error("server error")]
    ServerError(#[from] crate::errors::ServerError),
}

impl<'r> Responder<'r, 'static> for CreateChallengeError {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> response::Result<'static> {
        let status = match &self {
            Self::UserNotFound(_) => Status::Unauthorized,
            Self::ServerError(e) => {
                eprintln!("{e:?}");
                Status::InternalServerError
            }
        };
        let json = Json(json! ({"error": self.to_string()})).respond_to(request)?;
        Response::build_from(json).status(status).ok()
    }
}

#[derive(Error, Debug)]
pub enum EndSessionError {
    #[error("current user could not be identified")]
    UserNotFound,
    #[error("server error")]
    ServerError(#[from] ServerError),
}
impl<'r> Responder<'r, 'static> for EndSessionError {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> response::Result<'static> {
        let status = match &self {
            Self::UserNotFound => Status::BadRequest,
            Self::ServerError(e) => {
                eprintln!("{e:?}");
                Status::InternalServerError
            }
        };
        let json = Json(json! ({"error": self.to_string()})).respond_to(request)?;
        Response::build_from(json).status(status).ok()
    }
}
#[derive(Error, Debug)]
pub enum UserManagementError {
    #[error("user could not be identified as admin : {0}")]
    NotAdmin(String),
    #[error("No user found with email {0}")]
    UserDoesNotExist(String),
    #[error("Role {0} does not exist (either \"admin\" or\"waiter\")")]
    RoleDoesNotExist(String),
    #[error("A user cannot modify its role or delete itself")]
    UserCannotUpdateItSelf,
    #[error("server error")]
    ServerError(#[from] ServerError),
}
impl<'r> Responder<'r, 'static> for UserManagementError {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> response::Result<'static> {
        let status = match &self {
            Self::NotAdmin(_) => Status::Unauthorized,
            Self::UserDoesNotExist(_) | Self::RoleDoesNotExist(_) => Status::NotFound,
            UserManagementError::UserCannotUpdateItSelf => Status::BadRequest,
            Self::ServerError(e) => {
                eprintln!("{e:?}");
                Status::InternalServerError
            }
        };
        let json = Json(json! ({"error": self.to_string()})).respond_to(request)?;
        Response::build_from(json).status(status).ok()
    }
}

// #[derive(Error, Debug)]
// pub enum PaymentIntentError {
//     #[error("order with id {0} not found")]
//     OrderNotFound(u32),
//     #[error("error generating stripe payment intent")]
//     CouldNotGeneratePaymentIntent(#[from] stripe::StripeError),
//     #[error("could not retreive client secret in the created intent")]
//     NoClientSecretInIntent,
//     #[error("server error")]
//     ServerError(#[from] ServerError),
// }
// impl<'r> Responder<'r, 'static> for PaymentIntentError {
//     fn respond_to(self, request: &'r rocket::Request<'_>) -> response::Result<'static> {
//         let status = match &self {
//             Self::OrderNotFound(_) => Status::NotFound,
//             Self::NoClientSecretInIntent => Status::InternalServerError,
//             Self::ServerError(e) => {
//                 eprintln!("{e:?}");
//                 Status::InternalServerError
//             }
//             Self::CouldNotGeneratePaymentIntent(e) => {
//                 eprintln!("{e:?}");
//                 Status::InternalServerError
//             }
//         };
//         let json = Json(json! ({"error": self.to_string()})).respond_to(request)?;
//         Response::build_from(json).status(status).ok()
//     }
// }

#[derive(Error, Debug)]
pub enum PaymentIntentError {
    #[error("order with id {0} not found")]
    OrderNotFound(u32),
    #[error("could not create intent")]
    CouldNotCreateIntent,
    #[error("server error")]
    ServerError(#[from] ServerError),
}
impl<'r> Responder<'r, 'static> for PaymentIntentError {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> response::Result<'static> {
        let status = match &self {
            Self::OrderNotFound(_) => Status::NotFound,
            Self::CouldNotCreateIntent => Status::InternalServerError,
            Self::ServerError(e) => {
                eprintln!("{e:?}");
                Status::InternalServerError
            }
        };
        let json = Json(json! ({"error": self.to_string()})).respond_to(request)?;
        Response::build_from(json).status(status).ok()
    }
}
