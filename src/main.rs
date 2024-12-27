mod models;
mod utils;
mod repositories;
mod services;
mod controllers;
mod routes;

use axum::{serve, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().merge(routes::app_routes());

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");

    serve(listener, app).await.unwrap();
}