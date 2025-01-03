use axum::{Json, Extension};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use log::{info, error};
use crate::services::user_service::UserService;
use crate::controllers::errors::{ControllerError, ErrorResponse};
use crate::models::errors::{LoginError, RegistrationError};
use crate::utils::DbPool;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub pseudo: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub token: String,
}

pub async fn register_user(
    Extension(pool): Extension<DbPool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, ControllerError> {
    UserService::create_user(&pool, &payload).await.map(|_| {
        info!("User {} registered successfully.", payload.email);
        Json(RegisterResponse {
            message: "User registered successfully.".to_string(),
        })
    })
        .map_err(|e| {
            error!("Error registering user: {}", e);
            ControllerError::BadRequest(map_registration_error(e))
        })
}

pub async fn login_user_handler(
    Extension(pool): Extension<DbPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ControllerError> {
    UserService::login_user(&pool, &payload).await.map(|token| {
        info!("User {} logged in successfully.", payload.email);
        Json(LoginResponse {
            message: "User logged in successfully.".to_string(),
            token,
        })
    })
        .map_err(|e| {
            error!("Error logging in user: {}", e);
            ControllerError::BadRequest(map_login_error(e))
        })
}

fn map_registration_error(e: RegistrationError) -> ErrorResponse {
    match e {
        RegistrationError::EmailAlreadyTaken => ErrorResponse::new(
            "EMAIL_ALREADY_TAKEN".to_string(),
            "Email is already taken.".to_string(),
            Some("email".to_string()),
        ),
        RegistrationError::UsernameAlreadyTaken => ErrorResponse::new(
            "USERNAME_ALREADY_TAKEN".to_string(),
            "Username is already taken.".to_string(),
            Some("pseudo".to_string()),
        ),
        _ => ErrorResponse::new(
            "INTERNAL_SERVER_ERROR".to_string(),
            "Internal server error.".to_string(),
            None,
        ),
    }
}

fn map_login_error(e: LoginError) -> ErrorResponse {
    match e {
        LoginError::InvalidCredentials => ErrorResponse::new(
            "INVALID_CREDENTIALS".to_string(),
            "Email or password is incorrect.".to_string(),
            Some("email".to_string()),
        ),
        _ => ErrorResponse::new(
            "INTERNAL_SERVER_ERROR".to_string(),
            "Internal server error.".to_string(),
            None,
        ),
    }
}