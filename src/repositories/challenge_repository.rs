use diesel::prelude::*;
use diesel::result::Error;
use log::info;
use crate::models::challenge::{Challenge, NewChallenge};
use crate::schema::challenges;
use crate::schema::challenges::{category_id, name};

pub struct ChallengeRepository;

impl ChallengeRepository {
    pub async fn insert_challenge(conn: &mut PgConnection, new_challenge: &NewChallenge<'_>) -> Result<(), Error> {
        info!("Creating new challenge: {}", new_challenge.name);
        diesel::insert_into(challenges::table)
            .values(new_challenge)
            .execute(conn)?;
        info!("Challenge created successfully: {}", new_challenge.name);
        Ok(())
    }

    pub async fn delete_challenge(conn: &mut PgConnection, challenge_id: i32) -> Result<(), Error> {
        diesel::delete(challenges::table.find(challenge_id))
            .execute(conn)?;
        Ok(())
    }

    pub async fn find_challenge_by_name_and_category_id(
        conn: &mut PgConnection,
        challenge_name: &str,
        challenge_category_id: i32,
    ) -> Result<Option<Challenge>, Error> {
        challenges::table
            .filter(name.eq(challenge_name).and(category_id.eq(challenge_category_id)))
            .first::<Challenge>(conn)
            .optional()
    }

    pub fn get_all_challenges(conn: &mut PgConnection) -> Result<Vec<Challenge>, Error> {
        challenges::table.load::<Challenge>(conn)
    }
}