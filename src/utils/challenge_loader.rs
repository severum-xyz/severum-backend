use std::path::Path;
use serde::Deserialize;
use crate::services::category_service::CategoryService;
use crate::services::challenge_service::ChallengeService;
use crate::utils::get_db_connection;

#[derive(Deserialize)]
struct Metadata {
    id: String,
    challenge: ChallengeDetails,
}

#[derive(Deserialize)]
struct ChallengeDetails {
    title: String,
    category: String,
    difficulty: String,
    description: String,
    hint: Option<String>,
}

pub async fn load_challenges_from_repo(repo_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = get_db_connection().await.unwrap();

    for entry in walkdir::WalkDir::new(repo_path) {
        let entry = entry?;
        if entry.file_name() == "metadata.json" {
            let metadata: Metadata = serde_json::from_str(&std::fs::read_to_string(entry.path())?)?;

            let category_id = CategoryService::find_or_create_category(&mut conn, &metadata.challenge.category).await?;

            ChallengeService::find_or_create_challenge(
                &mut conn,
                &metadata.challenge.title,
                category_id,
                &metadata.challenge.difficulty,
                &metadata.challenge.description,
                metadata.challenge.hint.as_deref(),
            )
                .await?;
        }
    }

    Ok(())
}