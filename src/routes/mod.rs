use axum::Router;
use axum::routing::{get, post};
use crate::controllers::category_controller::{get_categories};
use crate::controllers::challenge_controller::{get_challenges, load_challenges};
use crate::controllers::user_controller::{login_user, register_user};
use crate::controllers::container_controller::{start_container, stop_container, list_containers, inspect_container};

pub fn user_routes() -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
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

pub fn container_routes() -> Router {
    Router::new()
        .route("/containers/start", post(start_container))
        .route("/containers/stop", post(stop_container))
        .route("/containers", get(list_containers))
        .route("/containers/:id", get(inspect_container))
}
