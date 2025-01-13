use std::env;

/// Retrieves the Docker image name from the `DOCKER_IMAGE` environment variable.
///
/// # Returns
/// A `String` containing the Docker image name. Defaults to `"0xmushow/severum:severum-sandbox-0.0.1"`
/// if the `DOCKER_IMAGE` variable is not set.
pub fn get_docker_image() -> String {
    env::var("DOCKER_IMAGE").unwrap_or_else(|_| "0xmushow/severum:severum-sandbox-0.0.1".to_string())
}
