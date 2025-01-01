use axum::{Json};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use log::{info, error};
use crate::services::user_service::UserService;
use crate::controllers::errors::ControllerError;
use crate::models::errors::{LoginError, RegistrationError};
use crate::utils::get_db_connection;

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

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn register_user(Json(payload): Json<RegisterRequest>) -> Result<impl IntoResponse, ControllerError> {
    let mut conn = get_db_connection().await;

    match UserService::create_user(&mut conn, &payload).await {
        Ok(_) => {
            info!("User {} registered successfully.", payload.email);
            Ok(Json(RegisterResponse {
                message: "User registered successfully.".to_string(),
            }))
        }
        Err(e) => {
            error!("Error: {}", e);
            let error_response = match e {
                RegistrationError::EmailAlreadyTaken => ErrorResponse {
                    error: ErrorDetail {
                        code: "EMAIL_ALREADY_TAKEN".to_string(),
                        message: "Email is already taken.".to_string(),
                        field: Some("email".to_string()),
                    },
                },
                RegistrationError::UsernameAlreadyTaken => ErrorResponse {
                    error: ErrorDetail {
                        code: "USERNAME_ALREADY_TAKEN".to_string(),
                        message: "Username is already taken.".to_string(),
                        field: Some("pseudo".to_string()),
                    },
                },
                _ => ErrorResponse {
                    error: ErrorDetail {
                        code: "INTERNAL_SERVER_ERROR".to_string(),
                        message: "Internal server error.".to_string(),
                        field: None,
                    },
                },
            };

            Err(ControllerError::BadRequest(error_response))
        }
    }
}

pub async fn login_user_handler(Json(payload): Json<LoginRequest>) -> Result<impl IntoResponse, ControllerError> {
    let mut conn = get_db_connection().await;

    match UserService::login_user(&mut conn, &payload).await {
        Ok(token) => {
            info!("User {} logged in successfully.", payload.email);
            Ok(Json(LoginResponse {
                message: "User logged in successfully.".to_string(),
                token,
            }))
        }
        Err(e) => {
            error!("Error logging in: {}", e);
            let error_response = match e {
                LoginError::InvalidCredentials => ErrorResponse {
                    error: ErrorDetail {
                        code: "INVALID_CREDENTIALS".to_string(),
                        message: "Email or password is incorrect.".to_string(),
                        field: Some("email".to_string()),
                    },
                },
                _ => ErrorResponse {
                    error: ErrorDetail {
                        code: "INTERNAL_SERVER_ERROR".to_string(),
                        message: "Internal server error.".to_string(),
                        field: None,
                    },
                },
            };

            Err(ControllerError::BadRequest(error_response))
        }
    }
}