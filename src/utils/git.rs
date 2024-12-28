use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
use dotenv::dotenv;
use std::env;
use std::path::Path;
use log::{info, error};

pub fn clone_or_update_repository(repo_url: &str, repo_path: &Path) {
    dotenv().ok();
    let github_pat = env::var("GIT_PRIVATE_TOKEN").expect("GIT_PRIVATE_TOKEN must be set in the environment");

    if repo_path.exists() {
        info!("Repository already exists. Attempting to fetch the latest changes...");
        match Repository::open(repo_path) {
            Ok(repo) => {
                fetch_latest_changes(&repo, &github_pat);
            }
            Err(e) => {
                error!("Failed to open existing repository: {}. Removing and recloning...", e);
                std::fs::remove_dir_all(repo_path).expect("Failed to remove invalid repository");
                clone_repository(repo_url, repo_path, &github_pat);
            }
        }
    } else {
        clone_repository(repo_url, repo_path, &github_pat);
    }
}

fn clone_repository(repo_url: &str, repo_path: &Path, github_pat: &str) {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _username_from_url, _allowed_types| {
        Cred::userpass_plaintext("oauth2", github_pat)
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    let mut repo_builder = git2::build::RepoBuilder::new();
    repo_builder.fetch_options(fetch_options);

    match repo_builder.clone(repo_url, repo_path) {
        Ok(_) => info!("Repository cloned successfully at: {}", repo_path.display()),
        Err(e) => error!("Failed to clone the repository: {}", e),
    }
}

fn fetch_latest_changes(repo: &Repository, github_pat: &str) {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, _username_from_url, _allowed_types| {
        Cred::userpass_plaintext("oauth2", github_pat)
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    match repo.find_remote("origin") {
        Ok(mut remote) => {
            if let Err(e) = remote.fetch(&["master"], Some(&mut fetch_options), None) {
                error!("Failed to fetch latest changes: {}", e);
            } else {
                info!("Repository updated successfully.");
            }
        }
        Err(e) => error!("Failed to find remote 'origin': {}", e),
    }
}
