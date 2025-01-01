use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    error: ErrorDetail,
}

#[derive(Serialize, Debug)]
pub struct ErrorDetail {
    code: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<String>,
}

#[derive(Debug)]
pub enum ControllerError {
    BadRequest(ErrorResponse),
    InternalServerError,
}

impl IntoResponse for ControllerError {
    fn into_response(self) -> Response {
        match self {
            ControllerError::BadRequest(error_response) => {
                (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
            }
            ControllerError::InternalServerError => {
                let error_response = ErrorResponse::new(
                    "INTERNAL_SERVER_ERROR".to_string(),
                    "Internal server error.".to_string(),
                    None,
                );
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
            }
        }
    }
}

impl ErrorResponse {
    pub fn new(code: String, message: String, field: Option<String>) -> Self {
        Self {
            error: ErrorDetail { code, message, field },
        }
    }
}

impl ErrorDetail {
    pub fn new(code: String, message: String, field: Option<String>) -> Self {
        Self { code, message, field }
    }
}