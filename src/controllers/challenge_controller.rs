use crate::services::challenge_service;
use crate::models::challenge::ChallengeMetadata;

pub async fn fetch_challenges(base_path: &str) -> Vec<ChallengeMetadata> {
    challenge_service::get_challenges(base_path)
}
