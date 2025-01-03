use sqlx::PgPool;
use crate::models::challenge::{Challenge, NewChallenge};
use crate::repositories::challenge_repository::ChallengeRepository;

pub struct ChallengeService;

impl ChallengeService {
    pub async fn get_all_challenges(pool: &PgPool) -> Result<Vec<Challenge>, sqlx::Error> {
        ChallengeRepository::get_all_challenges(pool).await
    }

    pub async fn find_or_create_challenge(
        pool: &PgPool,
        challenge_name: &str,
        category_id: i32,
        difficulty: &str,
        description: &str,
        hint: Option<&str>,
    ) -> Result<i32, sqlx::Error> {
        match ChallengeRepository::find_challenge_by_name_and_category_id(pool, challenge_name, category_id).await? {
            Some(existing_challenge) => Ok(existing_challenge.id),
            None => {
                let new_challenge = NewChallenge {
                    category_id,
                    name: challenge_name,
                    difficulty,
                    description,
                    hint,
                };
                ChallengeRepository::insert_challenge(pool, &new_challenge).await
            }
        }
    }
}
