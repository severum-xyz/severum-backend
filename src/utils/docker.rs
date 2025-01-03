use std::env;

pub fn get_docker_image() -> String {
    env::var("DOCKER_IMAGE").unwrap_or_else(|_| "0xmushow/severum:severum-sandbox-0.0.1".to_string())
}