use std::env;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

/// Type alias for a PostgreSQL connection pool.
pub type DbPool = Pool<Postgres>;

/// Creates a database connection pool using configuration from the environment.
///
/// This function uses the `DATABASE_URL` environment variable to establish a connection to the database.
///
/// # Returns
/// A `DbPool` instance for interacting with the database.
///
/// # Panics
/// This function panics if:
/// * The `DATABASE_URL` environment variable is not set.
/// * A connection pool cannot be created.
pub async fn create_db_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(200)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .idle_timeout(std::time::Duration::from_secs(600))
        .max_lifetime(Some(std::time::Duration::from_secs(1800)))
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}
