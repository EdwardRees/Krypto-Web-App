use entity::auth;
use sea_orm::{Set, ActiveModelTrait, EntityTrait, QueryFilter, ColumnTrait};
use uuid::Uuid;
use bcrypt::{hash};
use dotenv::dotenv;
use jsonwebtoken as jwt;
use std::env;
use time::{Duration};
use chrono::{Utc, TimeDelta, NaiveDateTime};

use db::{establish_connection};
use crate::structs::{_Claims, AuthCredentials, AuthError, ServerError};


fn create_token(user_id: &String, key: &String, exp: NaiveDateTime) -> String {
    let claims: _Claims = _Claims::new(user_id.to_string(), Utc::now().naive_utc().and_utc().timestamp() as usize, exp.and_utc().timestamp() as usize);
    let token = jwt::encode(&jwt::Header::default(), &claims, &jwt::EncodingKey::from_secret(key.as_ref())).unwrap();
    token
} 

pub async fn signup(email: String, password: String) -> Result<AuthCredentials, AuthError> {
    dotenv().ok();

    let email = &email;

    const HASHING_ROUNDS: u32 = 12;
    let access_token_secret: String = env::var("ACCESS_TOKEN_SECRET").expect("ACCESS_TOKEN_SECRET must be set");
    let refresh_token_secret: String = env::var("REFRESH_TOKEN_SECRET").expect("REFRESH_TOKEN_SECRET must be set");

    let db = match establish_connection().await {
        Ok(conn) => conn,
        Err(error) => {
            return Err(AuthError::Database(error));
        }
    };

    let user = auth::Entity::find().filter(auth::Column::Email.eq(email)).one(&db).await;
    match user {
        Ok(model) => match model {
            Some(_) => {
                return Err(AuthError::Server(ServerError::Conflict(String::from("user already exists!"))));
            },
            None => {}
        },
        Err(error) => {
            return Err(AuthError::Database(error));
        }
    };

    let hashed_password: Option<String> = match hash(&password, HASHING_ROUNDS) {
        Ok(hashed) => Some(hashed),
        Err(error) => {
            return Err(AuthError::Server(ServerError::InternalServerError(error.to_string())));
        }
    };

    let auth_user = auth::ActiveModel {
        id: Set(Uuid::new_v4()),
        email: Set(email.to_string()),
        password: Set(hashed_password.unwrap()),
        ..Default::default()
    };

    let user: Option<auth::Model> = match auth_user.insert(&db).await {
        Ok(model) => Some(model),
        Err(error) => {
            return Err(AuthError::Database(error));
        }
    };

    let user_id: String = user.as_ref().unwrap().id.to_string();

    let access_token: String = create_token(&user_id, &access_token_secret, Utc::now().naive_utc().checked_add_signed(TimeDelta::hours(2)).unwrap());
    let refresh_token: String = create_token(&user_id, &refresh_token_secret, Utc::now().naive_utc().checked_add_signed(TimeDelta::days(30)).unwrap());

    let response: AuthCredentials = AuthCredentials::new(user_id, access_token, refresh_token);

    Ok(response)
}
