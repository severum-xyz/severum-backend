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
            match e {
                RegistrationError::EmailAlreadyTaken | RegistrationError::UsernameAlreadyTaken => {
                    Err(ControllerError::BadRequest(e.to_string()))
                }
                _ => Err(ControllerError::InternalServerError),
            }
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
            match e {
                LoginError::InvalidCredentials => {
                    Err(ControllerError::BadRequest("Email or password is incorrect".to_string()))
                }
                _ => Err(ControllerError::InternalServerError),
            }
        }
    }
}