use thiserror::Error;
use sqlx::Error as SqlxError;

#[derive(Error, Debug)]
pub enum RegistrationError {
    #[error("Email is already taken")]
    EmailAlreadyTaken,

    #[error("Username is already taken")]
    UsernameAlreadyTaken,

    #[error("Internal server error")]
    InternalError,
}

impl From<SqlxError> for RegistrationError {
    fn from(_: SqlxError) -> Self {
        RegistrationError::InternalError
    }
}
