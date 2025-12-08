use serde::{Serialize, Deserialize};
use sea_orm::error::DbErr; 

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct _Claims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize
}

impl _Claims {
    pub fn new(sub: String, iat: usize, exp: usize) -> Self {
        let claim: _Claims = _Claims {
            sub, iat, exp
        };
        claim
    }
}

#[derive(Serialize, Deserialize)]
pub struct AuthCredentials {
    user_id: String,
    access_token: String,
    refresh_token: String
}

impl AuthCredentials {
    pub fn new(user_id: String, access_token: String, refresh_token: String) -> Self {
        AuthCredentials {
            user_id,
            access_token,
            refresh_token
        }
    }
}

pub enum ServerError {
    Conflict(String),
    InternalServerError(String),
    NotFound(String),
    Unauthorized(String)
}

pub enum AuthError {
    Database(DbErr),
    Server(ServerError)
}
