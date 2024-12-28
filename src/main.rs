mod models;
mod utils;
mod repositories;
mod services;
mod controllers;
mod routes;

use axum::{serve, Router};
use log::info;
use tokio::net::TcpListener;
use crate::utils::create_db_pool;

#[tokio::main]
async fn main() {
    env_logger::init();

    dotenv::dotenv().ok();
    let app = Router::new().merge(routes::app_routes());

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server running on http://0.0.0.0:3000");

    create_db_pool().await; // This will now log database connection attempts

    serve(listener, app).await.unwrap();
}
