mod models;
mod utils;
mod repositories;
mod services;
mod controllers;
mod routes;
pub mod schema;

use std::env;
use std::path::{Path, PathBuf};
use axum::{serve, Router};
use env_logger::init as log_init;
use log::{info, warn};
use tokio::net::TcpListener;
use crate::services::category_service::CategoryService;
use crate::utils::{challenge_loader, clone_or_update_repository, get_db_connection};

#[tokio::main]
async fn main() {
    log_init();

    init().await;
    run_server().await;
}

async fn init() {
    dotenv::dotenv().ok();
    init_git().await;
    populate_categories().await;
    populate_challenges().await;
}

async fn init_git() {
    let repo_url = env::var("REPO_URL").expect("REPO_URL must be set in the environment");
    let base_path = PathBuf::from(env::var("BASE_PATH").expect("BASE_PATH must be set in the environment"));
    clone_or_update_repository(&repo_url, &base_path);
}

async fn populate_categories() {
    let base_path = env::var("BASE_PATH").expect("BASE_PATH must be set in the environment");
    let repo_path = Path::new(&base_path);
    let mut conn = get_db_connection().await.unwrap_or_else(|e| {
        std::process::exit(1);
    });

    for entry in walkdir::WalkDir::new(repo_path) {
        let entry = entry.expect("Failed to read directory entry");
        if entry.file_name() == "metadata.json" {
            let metadata: serde_json::Value = serde_json::from_str(
                &std::fs::read_to_string(entry.path()).expect("Failed to read metadata.json")
            ).expect("Failed to parse metadata.json");

            let category_name = metadata["challenge"]["category"]
                .as_str()
                .expect("Category name not found in metadata");

            CategoryService::find_or_create_category(&mut conn, category_name).await.unwrap();
        }
    }
}

async fn populate_challenges() {
    let base_path = env::var("BASE_PATH").expect("BASE_PATH must be set in the environment");
    let repo_path = Path::new(&base_path);

    challenge_loader::load_challenges_from_repo(repo_path).await.unwrap();
}

async fn run_server() {
    let app = Router::new()
        .merge(routes::user_routes())
        .merge(routes::challenge_routes())
        .merge(routes::category_routes());

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server running on http://0.0.0.0:3000");

    serve(listener, app).await.unwrap();
}