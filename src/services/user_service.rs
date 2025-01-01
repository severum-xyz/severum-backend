use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use argon2::{self, Config};
use diesel::prelude::*;
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::{RngCore};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use crate::controllers::user_controller::{LoginRequest, RegisterRequest};
use crate::models::user::NewUser;
use crate::models::errors::{LoginError, RegistrationError};
use crate::repositories::user_repository;

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct UserService;

impl UserService {
    pub async fn create_user(conn: &mut PgConnection, payload: &RegisterRequest) -> Result<(), RegistrationError> {
        let password_hash = Self::hash_password(&payload.password)?;

        let new_user = NewUser {
            email: &payload.email,
            pseudo: &payload.pseudo,
            password_hash: &password_hash,
        };

        Self::insert_new_user(conn, &new_user).await?;
        Ok(())
    }

    fn generate_salt() -> [u8; 32] {
        let mut salt = [0u8; 32];
        OsRng.fill_bytes(&mut salt);
        salt
    }

    fn hash_password(password: &str) -> Result<String, RegistrationError> {
        let config = Config::default();
        let salt = Self::generate_salt();
        argon2::hash_encoded(password.as_bytes(), &salt, &config)
            .map_err(|_| RegistrationError::InternalError)
    }

    async fn insert_new_user(conn: &mut PgConnection, new_user: &NewUser<'_>) -> Result<(), RegistrationError> {
        let email_exists = user_repository::email_exists(conn, new_user.email)?;
        let pseudo_exists = user_repository::pseudo_exists(conn, new_user.pseudo)?;

        if email_exists {
            return Err(RegistrationError::EmailAlreadyTaken);
        }
        if pseudo_exists {
            return Err(RegistrationError::UsernameAlreadyTaken);
        }

        user_repository::insert_new_user(conn, new_user)?;
        Ok(())
    }

    pub async fn login_user(conn: &mut PgConnection, payload: &LoginRequest) -> Result<String, LoginError> {
        let jwt_secret = env::var("JWT_SECRET").map_err(|_| LoginError::InternalError)?;

        let user = user_repository::find_user_by_email(conn, &payload.email)?;

        match user {
            Some(u) if Self::verify_password(&payload.password, &u.password_hash) => {
                let claims = Claims {
                    sub: u.email,
                    exp: (SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs() + 86400) as usize,
                };

                encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(jwt_secret.as_ref()),
                )
                    .map_err(|_| LoginError::InternalError)
            }
            _ => Err(LoginError::InvalidCredentials)
        }
    }

    fn verify_password(password: &str, stored_hash: &str) -> bool {
        argon2::verify_encoded(stored_hash, password.as_bytes()).unwrap_or(false)
    }
}