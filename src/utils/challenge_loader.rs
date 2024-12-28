use crate::models::challenge::ChallengeMetadata;
use std::fs;
use std::path::Path;
use log::{debug, error, info};

pub fn load_challenges(base_path: &str) -> Vec<ChallengeMetadata> {
    info!("Loading challenges from base path: {}", base_path);
    let base_dir = Path::new(base_path);
    if !base_dir.exists() {
        error!("Base directory does not exist: {}", base_path);
        return vec![];
    }

    let mut challenges = Vec::new();
    scan_dir(base_dir, &mut challenges, base_path);
    challenges
}

fn scan_dir(dir: &Path, challenges: &mut Vec<ChallengeMetadata>, base_path: &str) {
    debug!("Scanning directory: {:?}", dir);
    let Ok(entries) = fs::read_dir(dir) else { return };

    for entry in entries {
        let Ok(entry) = entry else { continue };
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        if path.ends_with("data") {
            let metadata_path = path.join("metadata.json");
            let Ok(content) = fs::read_to_string(&metadata_path) else { continue };

            match serde_json::from_str::<ChallengeMetadata>(&content) {
                Ok(metadata) => {
                    challenges.push(metadata);
                }
                Err(e) => error!("Invalid metadata.json: {}", e),
            }
        }

        scan_dir(&path, challenges, base_path);
    }
}
