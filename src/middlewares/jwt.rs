use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Json},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;
use crate::models::claims::Claims;
use log::error;

pub async fn jwt_middleware(
    mut req: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    match extract_and_validate_token(&req).await {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            next.run(req).await
        }
        Err(response) => response.into_response(),
    }
}

async fn extract_and_validate_token(req: &Request<Body>) -> Result<Claims, (StatusCode, Json<serde_json::Value>)> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token = extract_token(req);
    match token {
        Some(token) => decode_token(&token, &jwt_secret),
        None => Err(missing_token_response()),
    }
}

fn extract_token(req: &Request<Body>) -> Option<String> {
    req.headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|auth| auth.strip_prefix("Bearer ").map(String::from))
}

fn decode_token(
    token: &str,
    jwt_secret: &str,
) -> Result<Claims, (StatusCode, Json<serde_json::Value>)> {
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    ) {
        Ok(decoded) => Ok(decoded.claims),
        Err(e) => {
            error!("Invalid JWT: {}", e);
            Err(invalid_token_response())
        }
    }
}

fn missing_token_response() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::UNAUTHORIZED,
        Json(json!({
            "error": "Missing token",
            "message": "Authorization header is missing or does not contain a Bearer token. Include a valid token in the Authorization header."
        })),
    )
}

fn invalid_token_response() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::UNAUTHORIZED,
        Json(json!({
            "error": "Invalid token",
            "message": "The provided JWT is invalid or expired. Please log in again to obtain a new token."
        })),
    )
}
