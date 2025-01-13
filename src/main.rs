//! Main module for the Severum backend server.
//!
//! This module initializes the application, connects to the database, configures
//! the Docker client, and starts the Axum web server. The application is structured
//! using an MVC pattern with middlewares, models, services, and routes.

mod middlewares;
mod models;
mod utils;
mod repositories;
mod services;
mod controllers;
mod routes;

use std::env;
use std::process::exit;
use std::sync::Arc;
use axum::{middleware, serve, Extension, Router};
use bollard::Docker;
use env_logger::{init as log_init};
use log::{error, info, warn};
use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::middlewares::jwt::jwt_middleware;
use crate::utils::db::create_db_pool;
use crate::utils::loader::Loader;

/// Shared application state used across the application.
///
/// Holds instances of the database connection pool, Docker client, and the
/// JWT secret required for user authentication and token validation.
pub struct AppState {
    /// PostgreSQL database connection pool.
    pub db_pool: PgPool,
    /// Docker client for managing container instances.
    pub docker_client: Docker,
    /// Secret used for signing and verifying JWTs.
    pub jwt_secret: String,
}

impl AppState {
    /// Creates a new instance of `AppState`.
    ///
    /// # Arguments
    ///
    /// * `db_pool` - A connection pool for the PostgreSQL database.
    /// * `docker_client` - A Docker client instance for managing containers.
    /// * `jwt_secret` - A string containing the JWT signing secret.
    pub fn new(db_pool: PgPool, docker_client: Docker, jwt_secret: String) -> Self {
        Self {
            db_pool,
            docker_client,
            jwt_secret,
        }
    }
}

/// Entry point for the backend application.
///
/// Initializes logging, sets up the database and Docker client, and starts the server.
#[tokio::main]
async fn main() {
    log_init();

    let db_pool = create_db_pool().await;

    let docker_client = match Docker::connect_with_local_defaults() {
        Ok(client) => client,
        Err(e) => {
            error!("Failed to connect to Docker: {}", e);
            warn!("Ensure Docker is installed, running, and accessible at /var/run/docker.sock.");
            exit(1);
        }
    };

    let state = Arc::new(
        AppState::new(
            db_pool,
            docker_client,
            env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
        ),
    );

    run_server(state).await;
}

/// Starts the Axum web server.
///
/// Configures the routes, middlewares, and initializes required application components.
///
/// # Arguments
///
/// * `state` - Shared application state containing the database pool,
///   Docker client, and JWT secret.
async fn run_server(state: Arc<AppState>) {
    Loader::init(&state.db_pool).await;

    // Define public (unauthenticated) routes
    let public_routes = Router::new()
        .merge(routes::user_routes())
        .merge(routes::category_routes());

    // Define protected (authenticated) routes with JWT middleware
    let protected_routes = Router::new()
        .merge(routes::challenge_routes())
        .merge(routes::container_routes())
        .layer(middleware::from_fn(jwt_middleware));

    let app = public_routes
        .merge(protected_routes)
        .layer(Extension(state.clone()));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server running on http://0.0.0.0:3000");

    serve(listener, app).await.unwrap();
}
