use axum::Router;
use axum::routing::post;
use crate::controllers::user_controller::{login_user_handler, register_user};

mod challenge_routes;

pub fn user_routes() -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user_handler))
}