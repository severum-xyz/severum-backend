use diesel::PgConnection;
use diesel::result::Error;
use crate::models::challenge::{Challenge, NewChallenge};
use crate::repositories::challenge_repository::ChallengeRepository;

pub struct ChallengeService;

impl ChallengeService {
    pub async fn find_or_create_challenge(
        conn: &mut PgConnection,
        challenge_name: &str,
        category_id: i32,
        difficulty: &str,
        description: &str,
        hint: Option<&str>,
    ) -> Result<i32, Error> {
        let challenge = ChallengeRepository::find_challenge_by_name_and_category_id(conn, challenge_name, category_id).await?;

        match challenge {
            Some(challenge) => Ok(challenge.id),
            None => {
                let new_challenge = NewChallenge {
                    category_id,
                    name: challenge_name,
                    difficulty,
                    description,
                    hint,
                };

                ChallengeRepository::insert_challenge(conn, &new_challenge).await?;

                let challenge = ChallengeRepository::find_challenge_by_name_and_category_id(conn, challenge_name, category_id).await?.unwrap();
                Ok(challenge.id)
            }
        }
    }

    pub fn get_all_challenges(conn: &mut PgConnection) -> Result<Vec<Challenge>, Error> {
        ChallengeRepository::get_all_challenges(conn)
    }
}