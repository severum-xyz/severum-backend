use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ControllerError {
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Internal server error")]
    InternalServerError,
}

impl IntoResponse for ControllerError {
    fn into_response(self) -> Response {
        let status_code = match self {
            ControllerError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ControllerError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(json!({
            "error": self.to_string(),
        }));

        (status_code, body).into_response()
    }
}