use sqlx::Error as SqlxError;
use jsonwebtoken::errors::Error as JwtError;
use thiserror::Error;
use log::error;

/// Represents errors that can occur during user registration.
#[derive(Error, Debug)]
pub enum RegistrationError {
    /// Error for when the email is already taken.
    #[error("Email is already taken")]
    EmailAlreadyTaken,

    /// Error for when the username is already taken.
    #[error("Username is already taken")]
    UsernameAlreadyTaken,

    /// Error for internal server issues during registration.
    #[error("Internal server error")]
    InternalError,
}

/// Represents errors that can occur during user login.
#[derive(Error, Debug)]
pub enum LoginError {
    /// Error for when the provided credentials are invalid.
    #[error("Email or password is incorrect")]
    InvalidCredentials,

    /// Error for internal server issues during login.
    #[error("Internal server error")]
    InternalError,
}

impl From<SqlxError> for RegistrationError {
    /// Converts a `SqlxError` into a `RegistrationError`.
    ///
    /// Logs the database error and returns an `InternalError` variant.
    fn from(e: SqlxError) -> Self {
        error!("Database error: {:?}", e);
        RegistrationError::InternalError
    }
}

impl From<SqlxError> for LoginError {
    /// Converts a `SqlxError` into a `LoginError`.
    ///
    /// Logs the database error and returns an `InternalError` variant.
    fn from(e: SqlxError) -> Self {
        error!("Database error: {:?}", e);
        LoginError::InternalError
    }
}

impl From<JwtError> for LoginError {
    /// Converts a `JwtError` into a `LoginError`.
    ///
    /// Logs the JWT error and returns an `InternalError` variant.
    fn from(e: JwtError) -> Self {
        error!("JWT error: {:?}", e);
        LoginError::InternalError
    }
}

/// Represents errors that can occur during the loading process.
#[derive(Debug)]
pub enum LoaderError {
    /// Error related to directory traversal.
    WalkDirError(()),

    /// Error related to input/output operations.
    IoError(()),

    /// Error related to JSON parsing or serialization.
    JsonError(()),

    /// Error related to database operations.
    DatabaseError(()),
}

impl From<walkdir::Error> for LoaderError {
    /// Converts a `walkdir::Error` into a `LoaderError`.
    fn from(_err: walkdir::Error) -> Self {
        LoaderError::WalkDirError(())
    }
}

impl From<std::io::Error> for LoaderError {
    /// Converts a `std::io::Error` into a `LoaderError`.
    fn from(_err: std::io::Error) -> Self {
        LoaderError::IoError(())
    }
}

impl From<serde_json::Error> for LoaderError {
    /// Converts a `serde_json::Error` into a `LoaderError`.
    fn from(_err: serde_json::Error) -> Self {
        LoaderError::JsonError(())
    }
}

impl From<sqlx::Error> for LoaderError {
    /// Converts a `sqlx::Error` into a `LoaderError`.
    fn from(_err: sqlx::Error) -> Self {
        LoaderError::DatabaseError(())
    }
}
