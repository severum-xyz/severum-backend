mod middlewares;
mod models;
mod utils;
mod repositories;
mod services;
mod controllers;
mod routes;
use std::env;
use std::path::{Path, PathBuf};
use axum::{middleware, serve, Extension, Router};
use env_logger::{init as log_init, init};
use log::{info, warn};
use tokio::net::TcpListener;
use crate::middlewares::jwt::jwt_middleware;
use crate::services::category_service::CategoryService;
use crate::utils::{clone_or_update_repository, create_db_pool};
use crate::utils::loader::Loader;

#[tokio::main]
async fn main() {
    log_init();

    run_server().await;
}

async fn run_server() {
    let pool = create_db_pool().await;
    Loader::init(&pool).await;

    let public_routes = Router::new()
        .merge(routes::user_routes())
        .merge(routes::category_routes());

    let protected_routes = Router::new()
        .merge(routes::challenge_routes())
        .layer(middleware::from_fn(jwt_middleware));

    let app = public_routes
        .merge(protected_routes)
        .layer(Extension(pool));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server running on http://0.0.0.0:3000");

    serve(listener, app).await.unwrap();
}