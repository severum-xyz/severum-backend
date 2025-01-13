use sqlx::PgPool;
use crate::models::challenge::{Challenge, NewChallenge};
use crate::repositories::challenge_repository::ChallengeRepository;

/// Service for managing challenges, including retrieval and creation.
pub struct ChallengeService;

impl ChallengeService {
    /// Retrieves all challenges from the database.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    ///
    /// # Returns
    /// A `Result` containing a vector of `Challenge` or a `sqlx::Error` if the query fails.
    pub async fn get_all_challenges(pool: &PgPool) -> Result<Vec<Challenge>, sqlx::Error> {
        ChallengeRepository::get_all_challenges(pool).await
    }

    /// Finds an existing challenge by its name and category or creates a new one if it doesn't exist.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    /// * `challenge_name` - The name of the challenge to find or create.
    /// * `category_id` - The ID of the category associated with the challenge.
    /// * `difficulty` - The difficulty level of the challenge.
    /// * `description` - A description of the challenge.
    /// * `hint` - An optional hint for the challenge.
    ///
    /// # Returns
    /// A `Result` containing the `Challenge` or a `sqlx::Error` if the operation fails.
    pub async fn find_or_create_challenge(
        pool: &PgPool,
        challenge_name: &str,
        category_id: i32,
        difficulty: &str,
        description: &str,
        hint: Option<&str>,
    ) -> Result<Challenge, sqlx::Error> {
        match ChallengeRepository::find_challenge_by_name_and_category_id(pool, challenge_name, category_id).await? {
            Some(existing_challenge) => Ok(existing_challenge),
            None => {
                let new_challenge = NewChallenge {
                    category_id,
                    name: challenge_name.to_string(),
                    difficulty: difficulty.to_string(),
                    description: description.to_string(),
                    hint: hint.map(|s| s.to_string()),
                };
                let challenge_id = ChallengeRepository::insert_challenge(pool, new_challenge).await?;
                let challenge = ChallengeRepository::find_challenge_by_id(pool, challenge_id).await?
                    .ok_or_else(|| sqlx::Error::RowNotFound)?;
                Ok(challenge)
            }
        }
    }
}
