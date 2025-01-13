use axum::Router;
use axum::routing::{get, post};
use crate::controllers::category_controller::get_categories;
use crate::controllers::challenge_controller::{get_challenges, load_challenges};
use crate::controllers::user_controller::{login_user, register_user};
use crate::controllers::container_controller::{
    start_container, stop_container, list_containers, inspect_container, create_container,
};

/// Defines routes related to user operations, such as registration and login.
///
/// # Routes
/// * `POST /register` - Registers a new user.
/// * `POST /login` - Logs in an existing user.
///
/// # Returns
/// A `Router` instance with user-related routes.
pub fn user_routes() -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
}

/// Defines routes related to challenges, such as fetching and loading challenges.
///
/// # Routes
/// * `POST /load-challenges` - Loads challenges into the system.
/// * `GET /challenges` - Retrieves a list of challenges.
///
/// # Returns
/// A `Router` instance with challenge-related routes.
pub fn challenge_routes() -> Router {
    Router::new()
        .route("/load-challenges", post(load_challenges))
        .route("/challenges", get(get_challenges))
}

/// Defines routes related to categories, such as fetching all categories.
///
/// # Routes
/// * `GET /categories` - Retrieves a list of categories.
///
/// # Returns
/// A `Router` instance with category-related routes.
pub fn category_routes() -> Router {
    Router::new()
        .route("/categories", get(get_categories))
}

/// Defines routes related to containers, such as creation, starting, stopping, and inspection.
///
/// # Routes
/// * `POST /containers/create` - Creates a new container.
/// * `POST /containers/start` - Starts a container.
/// * `POST /containers/stop` - Stops a container.
/// * `GET /containers` - Lists all containers.
/// * `GET /containers/{id}` - Inspects a specific container by ID.
///
/// # Returns
/// A `Router` instance with container-related routes.
pub fn container_routes() -> Router {
    Router::new()
        .route("/containers/create", post(create_container))
        .route("/containers/start", post(start_container))
        .route("/containers/stop", post(stop_container))
        .route("/containers", get(list_containers))
        .route("/containers/{id}", get(inspect_container))
}
