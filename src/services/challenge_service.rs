use crate::models::challenge::ChallengeMetadata;
use crate::repositories::challenge_repository;

pub fn get_challenges(base_path: &str) -> Vec<ChallengeMetadata> {
    challenge_repository::fetch_all_challenges(base_path)
}
