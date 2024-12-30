use crate::services::user_service::{create_user, login_user};
use crate::models::errors::{LoginError, RegistrationError};
use axum::{Json, http::StatusCode};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use log::{info, error};
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
    pub token: String, // JWT or session token
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn register_user(Json(payload): Json<RegisterRequest>) -> impl IntoResponse {
    let mut conn = get_db_connection().await;

    match create_user(&mut conn, &payload).await {
        Ok(_) => {
            info!("User {} registered successfully.", payload.email);
            Json(RegisterResponse {
                message: "User registered successfully.".to_string(),
            }).into_response()
        }
        Err(e) => {
            error!("Error: {}", e);
            let status_code = match e {
                RegistrationError::EmailAlreadyTaken | RegistrationError::UsernameAlreadyTaken => StatusCode::BAD_REQUEST,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };

            (status_code, Json(ErrorResponse {
                error: e.to_string(),
            })).into_response()
        }
    }
}

pub async fn login_user_handler(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    let mut conn = get_db_connection().await;

    match login_user(&mut conn, &payload).await {
        Ok(token) => {
            info!("User {} logged in successfully.", payload.email);
            Json(LoginResponse {
                message: "User logged in successfully.".to_string(),
                token,
            }).into_response()
        }
        Err(e) => {
            error!("Error logging in: {}", e);
            match e {
                LoginError::InvalidCredentials => {
                    (StatusCode::BAD_REQUEST, "Email or password is incorrect").into_response()
                }
                LoginError::InternalError => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
                }
            }
        }
    }
}