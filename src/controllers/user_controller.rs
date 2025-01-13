use std::sync::Arc;
use axum::{Json, Extension};
use serde::{Deserialize, Serialize};
use log::info;
use crate::AppState;
use crate::services::user_service::UserService;
use crate::controllers::errors::{ControllerError, ErrorResponse};
use crate::models::errors::{LoginError, RegistrationError};

/// Represents the request payload for registering a user.
#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

/// Represents the response for a successful user registration.
#[derive(Serialize)]
pub struct RegisterResponse {
    pub message: String,
}

/// Represents the request payload for logging in a user.
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Represents the response for a successful user login.
#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub token: String,
}

/// A generic type alias for JSON responses from controllers.
pub type JsonResponse<T> = Result<Json<T>, ControllerError>;

/// Handles user registration by creating a new user in the database.
///
/// # Arguments
/// * `state` - The shared application state, containing the database pool.
/// * `payload` - The JSON request payload containing the user's registration details.
///
/// # Returns
/// A `JsonResponse` containing either a success message or an error.
pub async fn register_user(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> JsonResponse<RegisterResponse> {
    let pool = &state.db_pool;
    UserService::create_user(&pool, &payload)
        .await
        .map(|_| {
            info!("User {} registered successfully.", payload.email);
            Json(RegisterResponse {
                message: "User registered successfully.".to_string(),
            })
        })
        .map_err(|e| ControllerError::BadRequest(map_registration_error(e)))
}

/// Handles user login by verifying credentials and returning a JWT.
///
/// # Arguments
/// * `state` - The shared application state, containing the database pool.
/// * `payload` - The JSON request payload containing the user's login details.
///
/// # Returns
/// A `JsonResponse` containing either a JWT token or an error.
pub async fn login_user(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> JsonResponse<LoginResponse> {
    let pool = &state.db_pool;
    UserService::login_user(&pool, &payload)
        .await
        .map(|token| {
            info!("User {} logged in successfully.", payload.username);
            Json(LoginResponse {
                message: "User logged in successfully.".to_string(),
                token,
            })
        })
        .map_err(|e| ControllerError::BadRequest(map_login_error(e)))
}

/// Maps registration errors to a user-friendly error response.
///
/// # Arguments
/// * `e` - The `RegistrationError` to map.
///
/// # Returns
/// An `ErrorResponse` describing the error in a user-friendly way.
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

/// Maps login errors to a user-friendly error response.
///
/// # Arguments
/// * `e` - The `LoginError` to map.
///
/// # Returns
/// An `ErrorResponse` describing the error in a user-friendly way.
fn map_login_error(e: LoginError) -> ErrorResponse {
    match e {
        LoginError::InvalidCredentials => ErrorResponse::new(
            "INVALID_CREDENTIALS".to_string(),
            "Invalid username or password.".to_string(),
            None,
        ),
        _ => ErrorResponse::new(
            "INTERNAL_SERVER_ERROR".to_string(),
            "Internal server error.".to_string(),
            None,
        ),
    }
}
