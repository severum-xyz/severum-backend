use axum::{Json, Extension};
use serde::Serialize;
use log::{info, error};
use crate::{
    services::category_service::CategoryService,
    controllers::errors::{ControllerError, ErrorResponse},
};
use crate::utils::db::DbPool;

#[derive(Serialize)]
pub struct CategoryResponse {
    id: i32,
    name: String,
}

pub async fn get_categories(
    Extension(pool): Extension<DbPool>,
) -> Result<Json<Vec<CategoryResponse>>, ControllerError> {
    info!("Fetching all categories...");

    let categories = CategoryService::get_all_categories(&pool).await.map_err(|e| {
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
