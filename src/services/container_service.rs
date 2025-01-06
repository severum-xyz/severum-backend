use crate::models::container::UserContainer;
use crate::models::role::Role;
use crate::repositories::container_repository::ContainerRepository;
use crate::repositories::user_repository::UserRepository;
use crate::controllers::errors::{ControllerError, ErrorResponse};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::claims::Claims;

pub struct ContainerService;

impl ContainerService {
    pub async fn create_container(
        pool: &PgPool,
        claims: &Claims,
        challenge_id: i32,
        category_id: i32,
    ) -> Result<UserContainer, ControllerError> {
        let user_id = Self::parse_user_id(claims)?;
        let role = Self::fetch_user_role(pool, user_id).await?;
        let containers = Self::fetch_user_containers(pool, user_id).await?;

        Self::check_container_creation_permission(role, &containers)?;

        let container_name = Uuid::new_v4();
        let container = Self::store_user_container(pool, user_id, container_name, challenge_id, category_id).await?;

        Ok(container)
    }

    fn parse_user_id(claims: &Claims) -> Result<i32, ControllerError> {
        claims.sub.parse::<i32>().map_err(|_| {
            ControllerError::BadRequest(ErrorResponse::new(
                "INVALID_USER_ID".to_string(),
                "Invalid user ID in claims".to_string(),
                None,
            ))
        })
    }

    async fn fetch_user_role(pool: &PgPool, user_id: i32) -> Result<Role, ControllerError> {
        let id = UserRepository::get_user_role(pool, user_id).await.map_err(|e| {
            ControllerError::InternalServerError(ErrorResponse::new(
                "DATABASE_ERROR".to_string(),
                format!("Failed to fetch user role: {}", e),
                None,
            ))
        })?;

        Role::from_id(id).ok_or_else(|| {
            ControllerError::InternalServerError(ErrorResponse::new(
                "INVALID_ROLE".to_string(),
                "Invalid role ID".to_string(),
                None,
            ))
        })
    }

    async fn fetch_user_containers(pool: &PgPool, user_id: i32) -> Result<Vec<UserContainer>, ControllerError> {
        ContainerRepository::get_user_containers(pool, user_id).await.map_err(|e| {
            ControllerError::InternalServerError(ErrorResponse::new(
                "DATABASE_ERROR".to_string(),
                format!("Failed to fetch containers: {}", e),
                None,
            ))
        })
    }

    fn check_container_creation_permission(role: Role, containers: &[UserContainer]) -> Result<(), ControllerError> {
        let max_containers = match role {
            Role::User => 1,
            Role::Vip | Role::Admin => 5,
        };

        if containers.len() >= max_containers {
            let error_message = match role {
                Role::User => "You can only have 1 active instance. Upgrade to VIP to run up to 5 instances.",
                Role::Vip | Role::Admin => "VIP users can run up to 5 instances. Contact support for additional capacity.",
            };

            return Err(ControllerError::BadRequest(ErrorResponse::new(
                "CONTAINER_LIMIT_REACHED".to_string(),
                error_message.to_string(),
                None,
            )));
        }

        Ok(())
    }

    async fn store_user_container(
        pool: &PgPool,
        user_id: i32,
        container_name: Uuid,
        challenge_id: i32,
        category_id: i32,
    ) -> Result<UserContainer, ControllerError> {
        ContainerRepository::store_user_container(
            pool,
            user_id,
            container_name,
            challenge_id,
            category_id,
        )
            .await
            .map_err(|e| {
                ControllerError::InternalServerError(ErrorResponse::new(
                    "DATABASE_ERROR".to_string(),
                    format!("Failed to store container: {}", e),
                    None,
                ))
            })
    }
}