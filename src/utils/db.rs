use sqlx::postgres::PgPoolOptions;
use std::env;
use urlencoding::encode;
use log::{info, error};

pub async fn create_db_pool() {
    let database_user = env::var("DATABASE_USER")
        .expect("DATABASE_USER must be set in .env");
    let database_name = env::var("DATABASE_NAME")
        .expect("DATABASE_NAME must be set in .env");
    let password = env::var("DATABASE_PASSWORD")
        .expect("DATABASE_PASSWORD must be set in .env");

    let encoded_password = encode(&password);

    let db_url = format!(
        "postgres://{}:{}@localhost/{}",
        database_user,
        encoded_password,
        database_name
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await;

    match pool {
        Ok(_) => info!("Successfully connected to the database!"),
        Err(e) => error!("Failed to connect to the database: {:?}", e),
    }
}
