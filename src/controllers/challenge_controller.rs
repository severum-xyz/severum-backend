use axum::{Json, response::IntoResponse};
use std::path::Path;
use log::{error, info};
use serde::Serialize;

use crate::{
    utils::{challenge_loader, get_db_connection},
    controllers::errors::{ControllerError, ErrorResponse},
    services::challenge_service::ChallengeService,
};

pub async fn load_challenges() -> Result<impl IntoResponse, ControllerError> {
    let repo_path = Path::new("/tmp/severum-challenges");

    match challenge_loader::load_challenges_from_repo(repo_path).await {
        Ok(_) => Ok(Json("Challenges loaded successfully.")),
        Err(e) => {
            let error_response = ErrorResponse::new(
                "INTERNAL_SERVER_ERROR".to_string(),
                format!("Failed to load challenges: {}", e),
                None,
            );
            Err(ControllerError::BadRequest(error_response))
        }
    }
}

#[derive(Serialize)]
pub struct ChallengeResponse {
    id: i32,
    category_id: i32,
    name: String,
    difficulty: String,
    description: String,
    hint: Option<String>,
}

pub async fn get_challenges() -> Result<Json<Vec<ChallengeResponse>>, ControllerError> {
    info!("Fetching all challenges...");

    let mut conn = get_db_connection().await;
    info!("Database connection established.");

    let challenges_list = tokio::task::spawn_blocking(move || {
        ChallengeService::get_all_challenges(&mut conn)
    })
        .await
        .map_err(|e| {
            error!("Internal server error: {}", e);
            let error_response = ErrorResponse::new(
                "INTERNAL_SERVER_ERROR".to_string(),
                "Failed to fetch challenges".to_string(),
                None,
            );
            ControllerError::InternalServerError(error_response)
        })?
        .map_err(|e| {
            error!("Database error: {}", e);
            let error_response = ErrorResponse::new(
                "DATABASE_ERROR".to_string(),
                "Failed to fetch challenges".to_string(),
                None,
            );
            ControllerError::InternalServerError(error_response)
        })?;

    let response = challenges_list
        .into_iter()
        .map(|challenge| ChallengeResponse {
            id: challenge.id,
            category_id: challenge.category_id,
            name: challenge.name,
            difficulty: challenge.difficulty,
            description: challenge.description,
            hint: challenge.hint,
        })
        .collect::<Vec<_>>();

    Ok(Json(response))
}