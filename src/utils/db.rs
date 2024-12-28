use std::collections::HashMap;
use sqlx::{Connection, PgConnection, Executor, PgPool, Row};
use sqlx::postgres::PgPoolOptions;
use std::env;
use urlencoding::encode;
use log::{info, error};
use crate::models::challenge::ChallengeMetadata;
use crate::utils::challenge_loader;

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
    let base_path = env::var("BASE_PATH").expect("BASE_PATH must be set in .env");
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
    populate_database(&pool, base_path.as_str()).await?;

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
            id SERIAL NOT NULL,
            category_id INT NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
            name TEXT NOT NULL,
            difficulty TEXT NOT NULL,
            description TEXT NOT NULL,
            hint TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
            PRIMARY KEY (category_id, id)
        )
        "#
    )
        .execute(pool)
        .await?;

    info!("Database tables initialized successfully.");
    Ok(())
}

pub async fn populate_database(pool: &PgPool, base_path: &str) -> Result<(), sqlx::Error> {
    info!("Populating database with challenges from base path: {}", base_path);

    let challenges = challenge_loader::load_challenges(base_path);

    if challenges.is_empty() {
        error!("No challenges found to populate the database.");
        return Ok(());
    }

    let mut categories_map: HashMap<String, Vec<ChallengeMetadata>> = HashMap::new();
    for challenge in challenges {
        categories_map
            .entry(challenge.challenge.category.clone())
            .or_default()
            .push(challenge);
    }

    for (category_name, challenges) in categories_map {
        let category_description = format!("Challenges related to {}", category_name);

        let category_id = insert_category(pool, &category_name, &category_description).await?;

        for challenge_metadata in challenges {
            let challenge_details = challenge_metadata.challenge;

            insert_challenge(
                pool,
                category_id,
                &challenge_details.title,
                &challenge_details.difficulty,
                &challenge_details.description,
                Some(&challenge_details.hint),
            )
                .await?;
        }
    }

    info!("Database population complete.");
    Ok(())
}

async fn insert_category(pool: &PgPool, name: &str, description: &str) -> Result<i32, sqlx::Error> {
    if let Some(row) = sqlx::query(
        r#"
        INSERT INTO categories (name, description)
        VALUES ($1, $2)
        ON CONFLICT (name) DO NOTHING
        RETURNING id
        "#
    )
        .bind(name)
        .bind(description)
        .fetch_optional(pool)
        .await?
    {
        return Ok(row.get::<i32, _>("id"));
    }

    let row = sqlx::query("SELECT id FROM categories WHERE name = $1")
        .bind(name)
        .fetch_one(pool)
        .await?;

    Ok(row.get::<i32, _>("id"))
}

async fn insert_challenge(
    pool: &PgPool,
    category_id: i32,
    name: &str,
    difficulty: &str,
    description: &str,
    hint: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO challenges (category_id, name, difficulty, description, hint)
        VALUES ($1, $2, $3, $4, $5)
        "#
    )
        .bind(category_id)
        .bind(name)
        .bind(difficulty)
        .bind(description)
        .bind(hint)
        .execute(pool)
        .await?;

    Ok(())
}
