use std::path::{Path, PathBuf};
use log::{info, error};
use serde::Deserialize;
use walkdir::WalkDir;
use crate::models::challenge::Challenge;
use crate::models::errors::LoaderError;
use crate::services::{category_service::CategoryService, challenge_service::ChallengeService};
use crate::utils::{clone_or_update_repository, db::DbPool};

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

pub struct Loader;

impl Loader {
    pub async fn init(pool: &DbPool) {
        dotenv::dotenv().ok();
        Self::init_git().await;
        Self::load_categories(pool).await;
        Self::load_challenges(pool).await;
    }

    async fn init_git() {
        let repo_url = std::env::var("REPO_URL").expect("REPO_URL must be set in the environment");
        let base_path = PathBuf::from(std::env::var("BASE_PATH").expect("BASE_PATH must be set in the environment"));
        clone_or_update_repository(&repo_url, &base_path);
        info!("Repository initialized.");
    }

    async fn load_categories(pool: &DbPool) {
        let base_path = std::env::var("BASE_PATH").expect("BASE_PATH must be set in the environment");
        let repo_path = Path::new(&base_path);

        for entry in WalkDir::new(repo_path) {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    error!("Failed to read directory entry: {}", e);
                    continue;
                }
            };

            if entry.file_name() == "metadata.json" {
                match std::fs::read_to_string(entry.path()) {
                    Ok(content) => {
                        if let Ok(metadata) = serde_json::from_str::<Metadata>(&content) {
                            if let Some(category_name) = Some(metadata.challenge.category.as_str()) {
                                match CategoryService::find_or_create_category(pool, category_name).await {
                                    Ok(_) => info!("Category '{}' loaded.", category_name),
                                    Err(e) => error!("Failed to create category '{}': {}", category_name, e),
                                }
                            } else {
                                error!("Category name not found in metadata: {:?}", entry.path());
                            }
                        } else {
                            error!("Failed to parse metadata.json: {:?}", entry.path());
                        }
                    }
                    Err(e) => {
                        error!("Failed to read metadata.json: {}", e);
                    }
                }
            }
        }
    }

    pub async fn load_challenges(pool: &DbPool) -> Result<Vec<Challenge>, LoaderError> {
        let base_path = std::env::var("BASE_PATH").expect("BASE_PATH must be set in the environment");
        let repo_path = Path::new(&base_path);
        let mut loaded_challenges = Vec::new();

        for entry in WalkDir::new(repo_path) {
            let entry = entry?;

            if entry.file_name() == "metadata.json" {
                let content = std::fs::read_to_string(entry.path())?;
                let metadata: Metadata = serde_json::from_str(&content)?;

                let category = CategoryService::find_or_create_category(
                    pool,
                    &metadata.challenge.category
                ).await?;

                let challenge = ChallengeService::find_or_create_challenge(
                    pool,
                    &metadata.challenge.title,
                    category.id,
                    &metadata.challenge.difficulty,
                    &metadata.challenge.description,
                    metadata.challenge.hint.as_deref(),
                ).await?;

                info!("Challenge '{}' loaded.", metadata.challenge.title);
                loaded_challenges.push(challenge);
            }
        }

        Ok(loaded_challenges)
    }
}
