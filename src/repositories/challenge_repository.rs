use sqlx::{PgPool, Error};
use log::info;
use crate::models::challenge::{Challenge, NewChallenge};

pub struct ChallengeRepository;

impl ChallengeRepository {
    pub async fn insert_challenge(pool: &PgPool, new_challenge: &NewChallenge<'_>) -> Result<i32, Error> {
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
