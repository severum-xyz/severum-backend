mod models;
mod utils;
mod repositories;
mod services;
mod controllers;
mod routes;

use axum::{serve, Router};
use env_logger::init as log_init;
use log::info;
use tokio::net::TcpListener;
use crate::utils::create_db_pool;

#[tokio::main]
async fn main() {
    log_init();

    init().await;
    run_server().await;
}

async fn init() {
    dotenv::dotenv().ok();
    create_db_pool().await;
}

async fn run_server() {
    let app = Router::new().merge(routes::app_routes());

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server running on http://0.0.0.0:3000");

    create_db_pool().await;

    serve(listener, app).await.unwrap();
}