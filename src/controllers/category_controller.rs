use axum::Json;
use serde::Serialize;
use log::{info, error};

use crate::{
    services::category_service::CategoryService,
    utils::get_db_connection,
    controllers::errors::{ControllerError, ErrorResponse},
};

#[derive(Serialize)]
pub struct CategoryResponse {
    id: i32,
    name: String,
}

pub async fn get_categories() -> Result<Json<Vec<CategoryResponse>>, ControllerError> {
    info!("Fetching all categories...");

    let mut conn = get_db_connection().await.unwrap();
    info!("Database connection established.");

    let categories = tokio::task::spawn_blocking(move || {
        CategoryService::get_all_categories(&mut conn)
    })
        .await
        .map_err(|e| {
            error!("Internal server error: {}", e);
            let error_response = ErrorResponse::new(
                "INTERNAL_SERVER_ERROR".to_string(),
                "Failed to fetch categories".to_string(),
                None,
            );
            ControllerError::InternalServerError(error_response)
        })?
        .map_err(|e| {
            error!("Database error: {}", e);
            let error_response = ErrorResponse::new(
                "DATABASE_ERROR".to_string(),
                "Failed to fetch categories".to_string(),
                None,
            );
            ControllerError::InternalServerError(error_response)
        })?;

    let response = categories
        .into_iter()
        .map(|category| CategoryResponse {
            id: category.id,
            name: category.name,
        })
        .collect::<Vec<_>>();

    Ok(Json(response))
}