mod middlewares;
mod models;
mod utils;
mod repositories;
mod services;
mod controllers;
mod routes;
use std::env;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::sync::Arc;
use axum::{middleware, serve, Extension, Router};
use bollard::Docker;
use env_logger::{init as log_init, init};
use log::{error, info, warn};
use sqlx::PgPool;
use tokio::net::TcpListener;
use crate::middlewares::jwt::jwt_middleware;
use crate::services::category_service::CategoryService;
use crate::utils::db::create_db_pool;
use crate::utils::loader::Loader;

pub struct AppState {
    pub db_pool: PgPool,
    pub docker_client: Docker,
    pub jwt_secret: String,
}

impl AppState {
    pub fn new(db_pool: PgPool, docker_client: Docker, jwt_secret: String) -> Self {
        Self {
            db_pool,
            docker_client,
            jwt_secret,
        }
    }
}

#[tokio::main]
async fn main() {
    log_init();

    let db_pool = create_db_pool().await;
    let docker_client = match Docker::connect_with_local_defaults() {
        Ok(client) => client,
        Err(e) => {
            error!("Failed to connect to Docker: {}", e);
            warn!(" Ensure Docker is installed, running, and accessible at /var/run/docker.sock.");
            exit(1);
        }
    };

    let state = Arc::new(
        AppState::new(
            db_pool,
            docker_client,
            env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set")));

    run_server(state).await;
}

async fn run_server(state: Arc<AppState>) {
    Loader::init(&state.db_pool).await;

    let public_routes = Router::new()
        .merge(routes::user_routes())
        .merge(routes::category_routes());

    let protected_routes = Router::new()
        .merge(routes::challenge_routes())
        .layer(middleware::from_fn(jwt_middleware));

    let app = public_routes
        .merge(protected_routes)
        .layer(Extension(state.clone()));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server running on http://0.0.0.0:3000");

    serve(listener, app).await.unwrap();
}