use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use argon2::{self, Config};
use diesel::prelude::*;
use diesel::{OptionalExtension, PgConnection, RunQueryDsl};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::{RngCore};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use crate::models::user::{NewUser, User};
use crate::models::errors::{LoginError, RegistrationError};
use crate::controllers::user_controller::{LoginRequest, RegisterRequest};
use crate::schema::users;

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn create_user(conn: &mut PgConnection, payload: &RegisterRequest) -> Result<(), RegistrationError> {
    let password_hash = hash_password(&payload.password);

    let new_user = NewUser {
        email: &payload.email,
        pseudo: &payload.pseudo,
        password_hash: &password_hash,
    };

    insert_new_user(conn, &new_user).await?;
    Ok(())
}

fn generate_salt() -> [u8; 32] {
    let mut salt = [0u8; 32];
    OsRng.fill_bytes(&mut salt);
    salt
}

fn hash_password(password: &str) -> String {
    let config = Config::default();
    let salt: [u8; 32] = generate_salt();
    argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap()
}

async fn insert_new_user(conn: &mut PgConnection, new_user: &NewUser<'_>) -> Result<(), RegistrationError> {
    let email_exists = diesel::select(diesel::dsl::exists(
        users::table.filter(users::email.eq(new_user.email))
    ))
        .get_result::<bool>(conn);

    let pseudo_exists = diesel::select(diesel::dsl::exists(
        users::table.filter(users::pseudo.eq(new_user.pseudo))
    ))
        .get_result::<bool>(conn);

    match email_exists {
        Ok(true) => return Err(RegistrationError::EmailAlreadyTaken),
        Ok(false) => {}
        Err(_) => return Err(RegistrationError::InternalError),
    }

    match pseudo_exists {
        Ok(true) => return Err(RegistrationError::UsernameAlreadyTaken),
        Ok(false) => {}
        Err(_) => return Err(RegistrationError::InternalError),
    }

    diesel::insert_into(users::table)
        .values(new_user)
        .execute(conn)?;

    Ok(())
}

pub async fn login_user(conn: &mut PgConnection, payload: &LoginRequest) -> Result<String, LoginError> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env");

    let user = users::table
        .filter(users::email.eq(&payload.email))
        .first::<User>(conn)
        .optional()
        .map_err(|_| LoginError::InternalError)?;

    match user {
        Some(u) if verify_password(&payload.password, &u.password_hash) => {
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