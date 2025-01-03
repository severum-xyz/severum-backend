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

#[derive(Debug)]
pub enum LoaderError {
    WalkDirError(walkdir::Error),
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    DatabaseError(sqlx::Error),
}

impl From<walkdir::Error> for LoaderError {
    fn from(err: walkdir::Error) -> Self {
        LoaderError::WalkDirError(err)
    }
}

impl From<std::io::Error> for LoaderError {
    fn from(err: std::io::Error) -> Self {
        LoaderError::IoError(err)
    }
}

impl From<serde_json::Error> for LoaderError {
    fn from(err: serde_json::Error) -> Self {
        LoaderError::JsonError(err)
    }
}

impl From<sqlx::Error> for LoaderError {
    fn from(err: sqlx::Error) -> Self {
        LoaderError::DatabaseError(err)
    }
}
