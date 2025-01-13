use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use argon2::{self, Config};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::{RngCore};
use rand::rngs::OsRng;
use crate::models::claims::Claims;
use crate::controllers::user_controller::{LoginRequest, RegisterRequest};
use crate::models::user::NewUser;
use crate::models::errors::{LoginError, RegistrationError};
use sqlx::PgPool;
use crate::repositories::user_repository::UserRepository;

const TOKEN_EXPIRATION_SECONDS: u64 = 86400; // 1 day
const SALT_LENGTH: usize = 32;

/// Service for managing user-related operations such as registration and login.
pub struct UserService;

impl UserService {
    /// Creates a new user in the database after validating their email and username.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    /// * `payload` - The registration details provided by the user.
    ///
    /// # Returns
    /// A `Result` indicating success or a `RegistrationError` if the operation fails.
    pub async fn create_user(pool: &PgPool, payload: &RegisterRequest) -> Result<(), RegistrationError> {
        let password_hash = Self::hash_password(&payload.password)?;

        let new_user = NewUser {
            email: payload.email.clone(),
            username: payload.username.clone(),
            password_hash,
        };

        Self::ensure_email_and_pseudo_unique(pool, &new_user).await?;
        UserRepository::insert_new_user(pool, new_user).await?;
        Ok(())
    }

    /// Authenticates a user and generates a JWT if the credentials are valid.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    /// * `payload` - The login details provided by the user.
    ///
    /// # Returns
    /// A `Result` containing the generated JWT or a `LoginError` if authentication fails.
    pub async fn login_user(pool: &PgPool, payload: &LoginRequest) -> Result<String, LoginError> {
        let jwt_secret = env::var("JWT_SECRET").map_err(|_| LoginError::InternalError)?;
        let user = UserRepository::find_user_by_username(pool, &payload.username)
            .await?
            .ok_or(LoginError::InvalidCredentials)?;

        if !Self::verify_password(&payload.password, &user.password_hash) {
            return Err(LoginError::InvalidCredentials);
        }

        let claims = Self::generate_claims(&user.id);
        encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_ref()))
            .map_err(|_| LoginError::InternalError)
    }

    /// Ensures that the email and username are unique in the database.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    /// * `new_user` - The details of the new user being registered.
    ///
    /// # Returns
    /// A `Result` indicating success or a `RegistrationError` if a duplicate is found.
    async fn ensure_email_and_pseudo_unique(pool: &PgPool, new_user: &NewUser) -> Result<(), RegistrationError> {
        if UserRepository::email_exists(pool, &new_user.email).await? {
            return Err(RegistrationError::EmailAlreadyTaken);
        }
        if UserRepository::username_exists(pool, &new_user.username).await? {
            return Err(RegistrationError::UsernameAlreadyTaken);
        }
        Ok(())
    }

    /// Generates JWT claims for a user.
    ///
    /// # Arguments
    /// * `user_identifier` - The ID of the user.
    ///
    /// # Returns
    /// A `Claims` struct containing the user's ID and token expiration time.
    fn generate_claims(user_identifier: &i32) -> Claims {
        Claims {
            sub: user_identifier.to_string(),
            exp: (SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() + TOKEN_EXPIRATION_SECONDS) as usize,
        }
    }

    /// Generates a random salt for password hashing.
    ///
    /// # Returns
    /// A 32-byte array representing the salt.
    fn generate_salt() -> [u8; SALT_LENGTH] {
        let mut salt = [0u8; SALT_LENGTH];
        OsRng.fill_bytes(&mut salt);
        salt
    }

    /// Hashes a password using Argon2.
    ///
    /// # Arguments
    /// * `password` - The plaintext password to hash.
    ///
    /// # Returns
    /// A `Result` containing the hashed password or a `RegistrationError` if hashing fails.
    fn hash_password(password: &str) -> Result<String, RegistrationError> {
        let config = Config::default();
        let salt = Self::generate_salt();
        argon2::hash_encoded(password.as_bytes(), &salt, &config)
            .map_err(|_| RegistrationError::InternalError)
    }

    /// Verifies a plaintext password against a stored hash.
    ///
    /// # Arguments
    /// * `password` - The plaintext password to verify.
    /// * `stored_hash` - The stored hash to compare against.
    ///
    /// # Returns
    /// A boolean indicating whether the password is valid.
    fn verify_password(password: &str, stored_hash: &str) -> bool {
        argon2::verify_encoded(stored_hash, password.as_bytes()).unwrap_or(false)
    }
}
