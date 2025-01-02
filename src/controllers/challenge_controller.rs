use axum::{Json, response::IntoResponse, Extension};
use log::{error, info};
use serde::Serialize;

use crate::{
    controllers::errors::{ControllerError, ErrorResponse},
    services::challenge_service::ChallengeService,
    utils::DbPool,
    utils::loader::Loader,
};

pub async fn load_challenges(Extension(pool): Extension<DbPool>) -> Result<impl IntoResponse, ControllerError> {
    Loader::load_challenges(&pool).await;
    Ok(Json("Challenges loaded successfully."))
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

pub async fn get_challenges(Extension(pool): Extension<DbPool>) -> Result<Json<Vec<ChallengeResponse>>, ControllerError> {
    info!("Fetching all challenges...");

    let challenges_list = ChallengeService::get_all_challenges(&pool).await.map_err(|e| {
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
