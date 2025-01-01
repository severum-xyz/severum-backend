use axum::Router;
use axum::routing::{get, post};
use crate::controllers::category_controller::{get_categories};
use crate::controllers::challenge_controller::{get_challenges, load_challenges};
use crate::controllers::user_controller::{login_user_handler, register_user};

pub fn user_routes() -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user_handler))
}

pub fn challenge_routes() -> Router {
    Router::new()
        .route("/load-challenges", post(load_challenges))
        .route("/challenges", get(get_challenges))
}

pub fn category_routes() -> Router {
    Router::new()
        .route("/categories", get(get_categories))
}