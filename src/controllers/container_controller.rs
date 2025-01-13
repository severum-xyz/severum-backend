use std::sync::Arc;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use log::info;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::AppState;
use crate::controllers::errors::ControllerError;
use crate::models::claims::Claims;
use crate::models::container::UserContainer;
use crate::services::container_service::ContainerService;

#[derive(Serialize, Deserialize)]
pub struct StartContainerRequest {
    pub challenge_id: i32,
    pub category_id: i32,
}

#[derive(Serialize)]
pub struct StartContainerResponse {
    pub container_id: i32,
    pub container_name: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
pub struct StopContainerRequest {
    pub id: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct StopContainerResponse {
    pub id: String,
    pub message: String,
    pub stopped_at: chrono::DateTime<chrono::Utc>,
}

/// Handles the creation of a new container for a user.
///
/// # Arguments
/// * `state` - Shared application state containing the database pool.
/// * `claims` - JWT claims of the user.
/// * `payload` - JSON payload containing challenge and category IDs.
///
/// # Returns
/// A `Result` with the created `UserContainer` or a `ControllerError` if the operation fails.
pub async fn create_container(
    Extension(state): Extension<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<StartContainerRequest>,
) -> Result<Json<UserContainer>, ControllerError> {
    let pool = &state.db_pool;

    let container = ContainerService::create_container(
        pool,
        &claims,
        payload.challenge_id,
        payload.category_id,
    ).await?;

    Ok(Json(container))
}

/// Handles the starting of a container.
///
/// # Returns
/// A placeholder response indicating the container was started successfully.
pub async fn start_container() -> Result<impl IntoResponse, ControllerError> {
    info!("Starting container...");
    Ok(Json(StartContainerResponse {
        container_id: 0,
        container_name: Default::default(),
        created_at: Default::default(),
    }))
}

/// Handles the stopping of a container.
///
/// # Returns
/// A placeholder response indicating the container was stopped successfully.
pub async fn stop_container() -> Result<impl IntoResponse, ControllerError> {
    info!("Stopping container...");
    Ok(Json(StopContainerResponse {
        id: "example_container_id".to_string(),
        message: "Container stopped successfully".to_string(),
        stopped_at: Default::default(),
    }))
}

#[derive(Serialize)]
struct ContainerResponse {
    id: String,
    message: String,
}

/// Lists all containers.
///
/// # Returns
/// A placeholder response containing a list of containers.
pub async fn list_containers() -> Result<impl IntoResponse, ControllerError> {
    info!("Listing containers...");
    Ok(Json(vec![
        ContainerResponse {
            id: "example_container_id".to_string(),
            message: "Container is running".to_string(),
        },
    ]))
}

/// Inspects a specific container.
///
/// # Returns
/// A placeholder response with container details.
pub async fn inspect_container() -> Result<impl IntoResponse, ControllerError> {
    info!("Inspecting container...");
    Ok(Json(ContainerResponse {
        id: "example_container_id".to_string(),
        message: "Container details retrieved successfully".to_string(),
    }))
}
