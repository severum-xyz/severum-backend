use std::path::{Path, PathBuf};
use log::{info, error};
use serde::Deserialize;
use walkdir::WalkDir;
use crate::models::challenge::Challenge;
use crate::models::errors::LoaderError;
use crate::services::{category_service::CategoryService, challenge_service::ChallengeService};
use crate::utils::db::DbPool;
use crate::utils::git::clone_or_update_repository;

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

/// Responsible for loading and initializing challenges and categories from a repository.
pub struct Loader;

impl Loader {
    /// Initializes the loader by setting up the repository, loading categories,
    /// and challenges into the database.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    pub async fn init(pool: &DbPool) {
        dotenv::dotenv().ok();
        Self::init_git().await;
        Self::load_categories(pool).await;
        if let Err(e) = Self::load_challenges(pool).await {
            error!("Failed to load challenges: {:?}", e);
        }
    }

    /// Clones or updates the repository containing challenge data.
    ///
    /// Expects the `REPO_URL` and `BASE_PATH` environment variables to be set.
    async fn init_git() {
        let repo_url = std::env::var("REPO_URL").expect("REPO_URL must be set in the environment");
        let base_path = PathBuf::from(std::env::var("BASE_PATH").expect("BASE_PATH must be set in the environment"));
        clone_or_update_repository(&repo_url, &base_path);
        info!("Repository initialized.");
    }

    /// Scans the repository for categories and loads them into the database.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
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
                                if let Err(e) = CategoryService::find_or_create_category(pool, category_name).await {
                                    error!("Failed to create category '{}': {}", category_name, e);
                                }
                            }
                        }
                    }
                    Err(e) => error!("Failed to read metadata.json: {}", e),
                }
            }
        }
    }

    /// Scans the repository for challenges and loads them into the database.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    ///
    /// # Returns
    /// * `Ok(Vec<Challenge>)` - A list of challenges that were successfully loaded.
    /// * `Err(LoaderError)` - An error occurred during the loading process.
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
                    &metadata.challenge.category,
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
