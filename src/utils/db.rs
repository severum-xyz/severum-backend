use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;
use log::{info, error};

pub async fn create_db_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    info!("Successfully connected to the PostgreSQL database.");

    pool
}
