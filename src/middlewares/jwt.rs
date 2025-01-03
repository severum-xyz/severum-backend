use axum::{
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use serde_json::{json, Value};
use std::env;
use axum::body::Body;
use axum::middleware::Next;
use jsonwebtoken::errors::Error;
use log::error;

pub async fn jwt_middleware(req: Request<Body>, next: Next) -> impl IntoResponse {
    let jwt_secret = get_jwt_secret();
    let token = extract_token_from_headers(&req);

    handle_token_validation(token, &jwt_secret, req, next).await
}

fn get_jwt_secret() -> String {
    env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

fn extract_token_from_headers(req: &Request<Body>) -> Option<String> {
    req.headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .map(|token| token.to_string())
}

async fn handle_token_validation(token: Option<String>, jwt_secret: &str, req: Request<Body>, next: Next) -> Response {
    match token {
        Some(token) => handle_valid_token(&token, jwt_secret, req, next).await,
        None => build_missing_token_response(),
    }
}

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

fn decode_token(token: &str, jwt_secret: &str) -> Result<TokenData<Value>, Error> {
    decode::<Value>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
}

fn build_missing_token_response() -> Response {
    build_error_response(
        StatusCode::UNAUTHORIZED,
        "Authorization Required",
        "No token provided. Please include a valid Bearer token in the Authorization header.",
    )
}

fn build_invalid_token_response() -> Response {
    build_error_response(
        StatusCode::UNAUTHORIZED,
        "Invalid JWT",
        "The provided token is invalid or expired. Please log in again to get a new token.",
    )
}

fn build_error_response(status_code: StatusCode, error: &str, message: &str) -> Response {
    let status_code = status_code;
    let body = Json(json!({ "error": error, "message": message }));

    (status_code, body).into_response()
}