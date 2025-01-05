use axum::{Json, response::IntoResponse, Extension};
use bollard::container::CreateContainerOptions;
use bollard::Docker;
use serde::Serialize;
use log::{info, error};
use sqlx::PgPool;
use crate::controllers::errors::ControllerError;
use crate::repositories::container_repository::ContainerRepository;
use crate::models::claims::Claims;

#[derive(Serialize)]
pub struct ContainerResponse {
    id: String,
    message: String,
}

pub async fn start_container() -> Result<impl IntoResponse, ControllerError> {
    info!("Stopping container...");
    // TODO: Implement container stop logic
    Ok(Json(ContainerResponse {
        id: "example_container_id".to_string(),
        message: "Container stopped successfully".to_string(),
    }))
}

/*
pub async fn start_container(
    Extension(docker): Extension<Docker>,          // Docker client
    Extension(pool): Extension<PgPool>,           // Database pool
    Extension(claims): Extension<Claims>,         // Extracted JWT claims
    ExtractJson(payload): ExtractJson<StartContainerRequest>, // JSON payload
) -> Result<Json<ContainerResponse>, ControllerError> {
    let user_id = claims.sub.parse::<i32>().unwrap(); // Extract user_id from JWT

    // Validate the challenge_id and category_id
    validate_challenge(&pool, payload.challenge_id, payload.category_id).await?;

    // Generate a unique name for the container
    let container_name = Uuid::new_v4();

    // Docker container configuration
    let config = Config {
        image: Some("0xmushow/severum:severum-sandbox-0.0.1".to_string()),
        ..Default::default()
    };

    // Create the container
    docker.create_container(
        Some(CreateContainerOptions {
            name: container_name.to_string(),
        }),
        config,
    ).await.map_err(|e| {
        error!("Failed to create container: {}", e);
        ControllerError::InternalServerError(e.to_string())
    })?;

    // Start the container
    docker.start_container(&container_name.to_string(), None).await.map_err(|e| {
        error!("Failed to start container: {}", e);
        ControllerError::InternalServerError(e.to_string())
    })?;

    // Store container details in the database
    ContainerRepository::insert_user_container(&pool, user_id, container_name, payload.challenge_id, payload.category_id).await?;

    // Return response
    Ok(Json(ContainerResponse {
        id: container_name.to_string(),
        message: "Container started successfully.".to_string(),
    }))
}
 */

pub async fn stop_container() -> Result<impl IntoResponse, ControllerError> {
    info!("Stopping container...");
    // TODO: Implement container stop logic
    Ok(Json(ContainerResponse {
        id: "example_container_id".to_string(),
        message: "Container stopped successfully".to_string(),
    }))
}

pub async fn list_containers() -> Result<impl IntoResponse, ControllerError> {
    info!("Listing containers...");
    // TODO: Implement container listing logic
    Ok(Json(vec![
        ContainerResponse {
            id: "example_container_id".to_string(),
            message: "Container is running".to_string(),
        },
    ]))
}

pub async fn inspect_container() -> Result<impl IntoResponse, ControllerError> {
    info!("Inspecting container...");
    // TODO: Implement container inspection logic
    Ok(Json(ContainerResponse {
        id: "example_container_id".to_string(),
        message: "Container details retrieved successfully".to_string(),
    }))
}
