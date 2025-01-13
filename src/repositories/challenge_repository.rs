use sqlx::{PgPool, Error};
use log::info;
use crate::models::challenge::{Challenge, NewChallenge};

/// Repository for managing challenges in the database.
pub struct ChallengeRepository;

impl ChallengeRepository {
    /// Inserts a new challenge into the `challenges` table.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    /// * `new_challenge` - A `NewChallenge` struct containing details of the challenge to insert.
    ///
    /// # Returns
    /// A `Result` containing the ID of the newly created challenge, or a `sqlx::Error` if the operation fails.
    pub async fn insert_challenge(pool: &PgPool, new_challenge: NewChallenge) -> Result<i32, Error> {
        info!("Creating new challenge: {}", new_challenge.name);

        let row: (i32,) = sqlx::query_as(
            r#"
            INSERT INTO challenges (category_id, name, difficulty, description, hint)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#
        )
            .bind(new_challenge.category_id)
            .bind(new_challenge.name)
            .bind(new_challenge.difficulty)
            .bind(new_challenge.description)
            .bind(new_challenge.hint)
            .fetch_one(pool)
            .await?;

        info!("Challenge created successfully with ID: {}", row.0);
        Ok(row.0)
    }

    /// Deletes a challenge from the `challenges` table by its ID.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    /// * `challenge_id` - The ID of the challenge to delete.
    ///
    /// # Returns
    /// A `Result` indicating success or a `sqlx::Error` if the operation fails.
    pub async fn delete_challenge(pool: &PgPool, challenge_id: i32) -> Result<(), Error> {
        sqlx::query(
            r#"
            DELETE FROM challenges WHERE id = $1
            "#
        )
            .bind(challenge_id)
            .execute(pool)
            .await?;

        info!("Challenge with ID {} deleted successfully", challenge_id);
        Ok(())
    }

    /// Finds a challenge by its name and category ID in the `challenges` table.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    /// * `challenge_name` - The name of the challenge to search for.
    /// * `challenge_category_id` - The ID of the category to which the challenge belongs.
    ///
    /// # Returns
    /// A `Result` containing an optional `Challenge` if found, or a `sqlx::Error` if the operation fails.
    pub async fn find_challenge_by_name_and_category_id(
        pool: &PgPool,
        challenge_name: &str,
        challenge_category_id: i32,
    ) -> Result<Option<Challenge>, Error> {
        sqlx::query_as::<_, Challenge>(
            r#"
            SELECT id, category_id, name, difficulty, description, hint, created_at
            FROM challenges
            WHERE name = $1 AND category_id = $2
            "#
        )
            .bind(challenge_name)
            .bind(challenge_category_id)
            .fetch_optional(pool)
            .await
    }

    /// Finds a challenge by its ID in the `challenges` table.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    /// * `challenge_id` - The ID of the challenge to search for.
    ///
    /// # Returns
    /// A `Result` containing an optional `Challenge` if found, or a `sqlx::Error` if the operation fails.
    pub async fn find_challenge_by_id(pool: &PgPool, challenge_id: i32) -> Result<Option<Challenge>, Error> {
        sqlx::query_as::<_, Challenge>(
            r#"
            SELECT id, category_id, name, difficulty, description, hint, created_at
            FROM challenges
            WHERE id = $1
            "#
        )
            .bind(challenge_id)
            .fetch_optional(pool)
            .await
    }

    /// Retrieves all challenges from the `challenges` table.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    ///
    /// # Returns
    /// A `Result` containing a vector of `Challenge` or a `sqlx::Error` if the operation fails.
    pub async fn get_all_challenges(pool: &PgPool) -> Result<Vec<Challenge>, Error> {
        sqlx::query_as::<_, Challenge>(
            r#"
            SELECT id, category_id, name, difficulty, description, hint, created_at
            FROM challenges
            "#
        )
            .fetch_all(pool)
            .await
    }
}
