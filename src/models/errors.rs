use sqlx::Error as SqlxError;
use jsonwebtoken::errors::Error as JwtError;
use thiserror::Error;
use log::error;

#[derive(Error, Debug)]
pub enum RegistrationError {
    #[error("Email is already taken")]
    EmailAlreadyTaken,

    #[error("Username is already taken")]
    UsernameAlreadyTaken,

    #[error("Internal server error")]
    InternalError,
}

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("Email or password is incorrect")]
    InvalidCredentials,

    #[error("Internal server error")]
    InternalError,
}

impl From<SqlxError> for RegistrationError {
    fn from(e: SqlxError) -> Self {
        error!("Database error: {:?}", e);
        RegistrationError::InternalError
    }
}

impl From<SqlxError> for LoginError {
    fn from(e: SqlxError) -> Self {
        error!("Database error: {:?}", e);
        LoginError::InternalError
    }
}

impl From<JwtError> for LoginError {
    fn from(e: JwtError) -> Self {
        error!("JWT error: {:?}", e);
        LoginError::InternalError
    }
}
