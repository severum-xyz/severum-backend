use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use thiserror::Error;

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

#[derive(Error, Debug)]
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
                let error_response = ErrorResponse {
                    error: ErrorDetail {
                        code: "INTERNAL_SERVER_ERROR".to_string(),
                        message: "Internal server error.".to_string(),
                        field: None,
                    },
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
            }
        }
    }
}