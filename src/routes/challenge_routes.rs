use axum::{Router, routing::get};
use axum::response::{IntoResponse, Json};
use crate::controllers::fetch_challenges;

pub fn routes() -> Router {
    Router::new()
        .route("/challenges", get(get_challenges))
}

pub async fn get_challenges() -> impl IntoResponse {
    let base_path = "/tmp/severum-challenges/";
    let challenges = fetch_challenges(base_path).await;
    Json(challenges)
}

