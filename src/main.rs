mod models;
mod utils;
mod repositories;
mod services;
mod controllers;
mod routes;

use std::env;
use std::path::PathBuf;
use axum::{serve, Router};
use env_logger::init as log_init;
use log::info;
use tokio::net::TcpListener;
use crate::utils::{clone_or_update_repository, initialize_database};

#[tokio::main]
async fn main() {
    log_init();

    init().await;
    run_server().await;
}

async fn init() {
    dotenv::dotenv().ok();
    init_git().await;
    init_database().await;
}

async fn init_git() {
    let repo_url = env::var("REPO_URL").expect("REPO_URL must be set in the environment");
    let base_path = PathBuf::from(env::var("BASE_PATH").expect("BASE_PATH must be set in the environment"));
    clone_or_update_repository(&repo_url, &base_path);
}

async fn init_database() {
    match initialize_database().await {
        Ok(_) => info!("Database initialized successfully."),
        Err(e) => eprintln!("Failed to initialize database: {}", e),
    }
}

async fn run_server() {
    let app = Router::new().merge(routes::app_routes());

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server running on http://0.0.0.0:3000");

    serve(listener, app).await.unwrap();
}