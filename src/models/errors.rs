use diesel::result::Error;
use jsonwebtoken::errors::Error as JwtError;
use log::error;
use thiserror::Error;

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

impl From<Error> for RegistrationError {
    fn from(e: Error) -> Self {
        error!("Database error: {:?}", e);
        RegistrationError::InternalError
    }
}

impl From<Error> for LoginError {
    fn from(e: Error) -> Self {
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