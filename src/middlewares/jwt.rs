use std::env;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Json;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use jsonwebtoken::errors::Error;
use log::error;
use serde_json::json;
use crate::models::claims::Claims;

/// Middleware for validating JWT tokens in incoming requests.
///
/// Extracts the token from the `Authorization` header, validates it, and
/// injects the decoded claims into the request extensions.
///
/// # Arguments
/// * `req` - The incoming HTTP request.
/// * `next` - The next middleware or handler in the chain.
///
/// # Returns
/// A response resulting from either the next handler or an error if the JWT validation fails.
pub async fn jwt_middleware(req: Request<Body>, next: Next) -> impl IntoResponse {
    let jwt_secret = get_jwt_secret();
    let token = extract_token_from_headers(&req);

    handle_token_validation(token, &jwt_secret, req, next).await
}

/// Retrieves the JWT secret from the environment variables.
///
/// # Panics
/// Panics if `JWT_SECRET` is not set in the environment.
fn get_jwt_secret() -> String {
    env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

/// Extracts the Bearer token from the `Authorization` header of a request.
///
/// # Arguments
/// * `req` - A reference to the HTTP request.
///
/// # Returns
/// An optional string containing the token if the header is present and correctly formatted.
fn extract_token_from_headers(req: &Request<Body>) -> Option<String> {
    req.headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .map(|token| token.to_string())
}

/// Handles the validation of a token and executes the next middleware or handler if valid.
///
/// # Arguments
/// * `token` - An optional string containing the extracted token.
/// * `jwt_secret` - The JWT secret used for token validation.
/// * `req` - The incoming HTTP request.
/// * `next` - The next middleware or handler in the chain.
///
/// # Returns
/// A response indicating the result of token validation or the next handler's response.
async fn handle_token_validation(token: Option<String>, jwt_secret: &str, req: Request<Body>, next: Next) -> Response {
    match token {
        Some(token) => handle_valid_token(&token, jwt_secret, req, next).await,
        None => build_missing_token_response(),
    }
}

/// Validates the token and injects claims into the request extensions if valid.
///
/// # Arguments
/// * `token` - A reference to the token string.
/// * `jwt_secret` - The JWT secret used for validation.
/// * `req` - The incoming HTTP request, which is mutated to include decoded claims.
/// * `next` - The next middleware or handler in the chain.
///
/// # Returns
/// A response indicating the result of validation or the next handler's response.
async fn handle_valid_token(token: &str, jwt_secret: &str, mut req: Request<Body>, next: Next) -> Response {
    match decode_token(token, jwt_secret) {
        Ok(decoded) => {
            req.extensions_mut().insert(decoded.claims);
            next.run(req).await
        }
        Err(e) => {
            error!("Invalid JWT: {}", e);
            build_invalid_token_response()
        }
    }
}

/// Decodes a JWT token into claims.
///
/// # Arguments
/// * `token` - The token string to decode.
/// * `jwt_secret` - The secret used for decoding the token.
///
/// # Returns
/// A `Result` containing the decoded token data or an error.
fn decode_token(token: &str, jwt_secret: &str) -> Result<TokenData<Claims>, Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
}

/// Builds a response for a missing token scenario.
///
/// # Returns
/// A `Response` with a 401 Unauthorized status and a JSON error message.
fn build_missing_token_response() -> Response {
    build_error_response(
        StatusCode::UNAUTHORIZED,
        "Authorization Required",
        "No token provided. Please include a valid Bearer token in the Authorization header.",
    )
}

/// Builds a response for an invalid token scenario.
///
/// # Returns
/// A `Response` with a 401 Unauthorized status and a JSON error message.
fn build_invalid_token_response() -> Response {
    build_error_response(
        StatusCode::UNAUTHORIZED,
        "Invalid JWT",
        "The provided token is invalid or expired. Please log in again to get a new token.",
    )
}

/// Constructs a generic error response with a JSON body.
///
/// # Arguments
/// * `status_code` - The HTTP status code for the response.
/// * `error` - A brief description of the error.
/// * `message` - A detailed error message.
///
/// # Returns
/// A `Response` with the specified status and JSON body.
fn build_error_response(status_code: StatusCode, error: &str, message: &str) -> Response {
    let body = Json(json!({ "error": error, "message": message }));
    (status_code, body).into_response()
}
