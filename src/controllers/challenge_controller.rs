/*
use crate::services::challenge_service::get_challenges;
use crate::models::challenge::ChallengeMetadata;

pub async fn fetch_challenges(base_path: &str) -> Vec<ChallengeMetadata> {
    get_challenges(base_path)
}

pub async fn fetch_challenge_by_id(base_path: &str, id: &str) -> Option<ChallengeMetadata> {
    let challenges = get_challenges(base_path);
    challenges.into_iter().find(|challenge| challenge.id == id)
}

 */