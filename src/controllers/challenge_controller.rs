use axum::{Json, response::IntoResponse, Extension};
use log::{error, info};
use serde::Serialize;

use crate::{
    controllers::errors::{ControllerError, ErrorResponse},
    services::challenge_service::ChallengeService,
    utils::{DbPool, loader::Loader},
};

#[derive(Serialize)]
pub struct ChallengeResponse {
    id: i32,
    category_id: i32,
    name: String,
    difficulty: String,
    description: String,
    hint: Option<String>,
}

pub async fn load_challenges(Extension(pool): Extension<DbPool>) -> Result<impl IntoResponse, ControllerError> {
    Loader::load_challenges(&pool).await.map_err(|e| {
        error!("Error loading challenges: {:?}", e);
        ControllerError::InternalServerError(ErrorResponse::new(
            "LOADER_ERROR".to_string(),
            "Failed to load challenges".to_string(),
            None,
        ))
    })?;

    Ok(Json("Challenges loaded successfully."))
}

pub async fn get_challenges(Extension(pool): Extension<DbPool>) -> Result<Json<Vec<ChallengeResponse>>, ControllerError> {
    info!("Fetching all challenges...");

    let challenges = ChallengeService::get_all_challenges(&pool).await.map_err(|e| {
        error!("Database error: {:?}", e);
        ControllerError::InternalServerError(ErrorResponse::new(
            "DATABASE_ERROR".to_string(),
            "Failed to fetch challenges".to_string(),
            None,
        ))
    })?;

    let response = challenges
        .into_iter()
        .map(|challenge| ChallengeResponse {
            id: challenge.id,
            category_id: challenge.category_id,
            name: challenge.name,
            difficulty: challenge.difficulty,
            description: challenge.description,
            hint: challenge.hint,
        })
        .collect();

    Ok(Json(response))
}
