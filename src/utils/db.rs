use sqlx::{Connection, PgConnection, Executor, PgPool};
use sqlx::postgres::PgPoolOptions;
use std::env;
use urlencoding::encode;
use log::{info, error};

pub fn get_database_url(db_name: &str) -> String {
    let database_user = env::var("DATABASE_USER").expect("DATABASE_USER must be set in .env");
    let database_password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set in .env");
    let database_host = env::var("DATABASE_HOST").unwrap_or_else(|_| "localhost".to_string());

    let encoded_password = encode(&database_password);

    format!(
        "postgres://{}:{}@{}/{}",
        database_user, encoded_password, database_host, db_name
    )
}

async fn connect_to_postgres(base_database_url: &str) -> Result<PgConnection, sqlx::Error> {
    match PgConnection::connect(base_database_url).await {
        Ok(conn) => {
            info!("Successfully connected to the PostgreSQL server.");
            Ok(conn)
        }
        Err(e) => {
            error!("Failed to connect to the PostgreSQL server: {:?}", e);
            Err(e)
        }
    }
}

async fn check_database_exists(connection: &mut PgConnection, database_name: &str) -> Result<bool, sqlx::Error> {
    let query = "SELECT 1 FROM pg_database WHERE datname = $1";
    let database_exists: Option<(i32,)> = sqlx::query_as(query)
        .bind(database_name)
        .fetch_optional(connection)
        .await?;

    Ok(database_exists.is_some())
}

async fn create_database(connection: &mut PgConnection, database_name: &str) -> Result<(), sqlx::Error> {
    info!("Database '{}' does not exist. Creating it...", database_name);
    let create_query = format!("CREATE DATABASE \"{}\"", database_name);
    connection.execute(create_query.as_str()).await?;
    info!("Database '{}' created successfully.", database_name);
    Ok(())
}

async fn connect_to_target_database(target_database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(target_database_url)
        .await?;

    info!("Successfully connected to the target database.");
    Ok(pool)
}

pub async fn initialize_database() -> Result<PgPool, sqlx::Error> {
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set in .env");
    let base_database_url = get_database_url("postgres");

    let mut base_connection = connect_to_postgres(&base_database_url).await?;

    if !check_database_exists(&mut base_connection, &database_name).await? {
        create_database(&mut base_connection, &database_name).await?;
    } else {
        info!("Database '{}' already exists.", database_name);
    }

    let target_database_url = get_database_url(&database_name);
    let pool = connect_to_target_database(&target_database_url).await?;
    initialize_tables(&pool).await?;

    Ok(pool)
}
async fn initialize_tables(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS categories (
            id SERIAL PRIMARY KEY,
            name TEXT UNIQUE NOT NULL,
            description TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
        )
        "#
    )
        .execute(pool)
        .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS challenges (
            id SERIAL PRIMARY KEY,
            category_id INT NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            difficulty TEXT NOT NULL,
            description TEXT NOT NULL,
            hint TEXT,
            src_path TEXT NOT NULL,
            exploit_path TEXT NOT NULL,
            test_path TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
        )
        "#
    )
        .execute(pool)
        .await?;

    info!("Database tables initialized successfully.");
    Ok(())
}