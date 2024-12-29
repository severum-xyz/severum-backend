use crate::models::user::{NewUser};
use crate::models::errors::RegistrationError;
use argon2::{self, Config};
use rand::{RngCore};
use rand::rngs::OsRng;
use sqlx::{PgPool};
use crate::controllers::user_controller::RegisterRequest;

pub async fn create_user(pool: &PgPool, payload: &RegisterRequest) -> Result<(), RegistrationError> {
    let password_hash = hash_password(&payload.password);

    let new_user = NewUser {
        email: &payload.email,
        pseudo: &payload.pseudo,
        password_hash: &password_hash,
    };

    insert_new_user(pool, &new_user).await?;
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

async fn insert_new_user(pool: &PgPool, new_user: &NewUser<'_>) -> Result<(), RegistrationError> {
    let email_exists = sqlx::query("SELECT 1 FROM users WHERE email = $1")
        .bind(new_user.email)
        .fetch_optional(pool)
        .await?;

    let pseudo_exists = sqlx::query("SELECT 1 FROM users WHERE pseudo = $1")
        .bind(new_user.pseudo)
        .fetch_optional(pool)
        .await?;

    if email_exists.is_some() {
        return Err(RegistrationError::EmailAlreadyTaken);
    }

    if pseudo_exists.is_some() {
        return Err(RegistrationError::UsernameAlreadyTaken);
    }

    sqlx::query(
        r#"
        INSERT INTO users (email, pseudo, password_hash)
        VALUES ($1, $2, $3)
        "#,
    )
        .bind(new_user.email)
        .bind(new_user.pseudo)
        .bind(new_user.password_hash)
        .execute(pool)
        .await?;

    Ok(())
}
