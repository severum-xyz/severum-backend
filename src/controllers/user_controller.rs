use crate::services::user_service::create_user;
use crate::models::errors::RegistrationError;
use axum::{Json, http::StatusCode};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use log::{info, error};
use crate::utils::create_db_pool;

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

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn register_user(Json(payload): Json<RegisterRequest>) -> impl IntoResponse {
    let pool = create_db_pool().await;

    match create_user(&pool, &payload).await {
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
