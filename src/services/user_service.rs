use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use argon2::{self, Config};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::{RngCore};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use crate::controllers::user_controller::{LoginRequest, RegisterRequest};
use crate::models::user::NewUser;
use crate::models::errors::{LoginError, RegistrationError};
use sqlx::PgPool;
use crate::repositories::user_repository::UserRepository;

const TOKEN_EXPIRATION_SECONDS: u64 = 86400; // 1 day
const SALT_LENGTH: usize = 32;

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct UserService;

impl UserService {
    pub async fn create_user(pool: &PgPool, payload: &RegisterRequest) -> Result<(), RegistrationError> {
        let password_hash = Self::hash_password(&payload.password)?;

        let new_user = NewUser {
            email: &payload.email,
            pseudo: &payload.pseudo,
            password_hash: &password_hash,
        };

        Self::ensure_email_and_pseudo_unique(pool, &new_user).await?;
        UserRepository::insert_new_user(pool, &new_user).await?;
        Ok(())
    }

    pub async fn login_user(pool: &PgPool, payload: &LoginRequest) -> Result<String, LoginError> {
        let jwt_secret = env::var("JWT_SECRET").map_err(|_| LoginError::InternalError)?;
        let user = UserRepository::find_user_by_email(pool, &payload.email)
            .await?
            .ok_or(LoginError::InvalidCredentials)?;

        if !Self::verify_password(&payload.password, &user.password_hash) {
            return Err(LoginError::InvalidCredentials);
        }

        let claims = Self::generate_claims(&user.email);
        encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_ref()))
            .map_err(|_| LoginError::InternalError)
    }

    async fn ensure_email_and_pseudo_unique(pool: &PgPool, new_user: &NewUser<'_>) -> Result<(), RegistrationError> {
        if UserRepository::email_exists(pool, new_user.email).await? {
            return Err(RegistrationError::EmailAlreadyTaken);
        }
        if UserRepository::pseudo_exists(pool, new_user.pseudo).await? {
            return Err(RegistrationError::UsernameAlreadyTaken);
        }
        Ok(())
    }

    fn generate_claims(email: &str) -> Claims {
        Claims {
            sub: email.to_string(),
            exp: (SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() + TOKEN_EXPIRATION_SECONDS) as usize,
        }
    }

    fn generate_salt() -> [u8; SALT_LENGTH] {
        let mut salt = [0u8; SALT_LENGTH];
        OsRng.fill_bytes(&mut salt);
        salt
    }

    fn hash_password(password: &str) -> Result<String, RegistrationError> {
        let config = Config::default();
        let salt = Self::generate_salt();
        argon2::hash_encoded(password.as_bytes(), &salt, &config)
            .map_err(|_| RegistrationError::InternalError)
    }

    fn verify_password(password: &str, stored_hash: &str) -> bool {
        argon2::verify_encoded(stored_hash, password.as_bytes()).unwrap_or(false)
    }
}
