use axum::{
    Router,
    routing::get,
    response::{IntoResponse, Json},
    extract::Path,
    http::StatusCode,
};
use log::info;
use crate::controllers::{fetch_challenges, fetch_challenge_by_id};

const BASE_PATH: &str = "/tmp/severum-challenges/";

pub fn routes() -> Router {
    Router::new()
        .route("/challenges", get(get_challenges))
        .route("/challenges/:id", get(get_challenge))
}

pub async fn get_challenges() -> impl IntoResponse {
    let challenges = fetch_challenges(BASE_PATH).await;
    Json(challenges)
}

pub async fn get_challenge(Path(id): Path<String>) -> impl IntoResponse {
    let challenge = fetch_challenge_by_id(BASE_PATH, &id).await;
    match challenge {
        Some(challenge) => {
            info!("Challenge found: ID: {}, Title: {}", id, challenge.challenge.title);
            Json(challenge).into_response()
        },
        None => {
            info!("Challenge not found: ID: {}", id);
            (StatusCode::NOT_FOUND, "Challenge not found").into_response()
        }
    }
}
