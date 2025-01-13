use crate::models::container::{NewContainer, UserContainer};
use sqlx::{Error, PgPool};
use uuid::Uuid;

/// Repository for interacting with the `user_containers` table in the database.
pub struct ContainerRepository;

impl ContainerRepository {
    /// Retrieves all containers from the `user_containers` table.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    ///
    /// # Returns
    /// A `Result` containing a vector of `UserContainer` or a `sqlx::Error` if the query fails.
    pub async fn get_all_containers(pool: &PgPool) -> Result<Vec<UserContainer>, Error> {
        sqlx::query_as::<_, UserContainer>(
            r#"
        SELECT id, user_id, container_name, created_at
        FROM user_containers
        "#
        )
            .fetch_all(pool)
            .await
    }

    /// Retrieves all containers associated with a specific user.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    /// * `user_id` - The ID of the user whose containers are to be fetched.
    ///
    /// # Returns
    /// A `Result` containing a vector of `UserContainer` or a `sqlx::Error` if the query fails.
    pub async fn get_user_containers(pool: &PgPool, user_id: i32) -> Result<Vec<UserContainer>, Error> {
        sqlx::query_as::<_, UserContainer>(
            r#"
        SELECT id, user_id, challenge_id, category_id, container_name, created_at
        FROM user_containers
        WHERE user_id = $1
        "#
        )
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    /// Finds all containers for a user by their user ID.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    /// * `user_id` - The ID of the user whose containers are to be fetched.
    ///
    /// # Returns
    /// A `Result` containing a vector of `UserContainer` or a `sqlx::Error` if the query fails.
    pub async fn find_container_by_user_id(
        pool: &PgPool,
        user_id: i32,
    ) -> Result<Vec<UserContainer>, Error> {
        sqlx::query_as::<_, UserContainer>(
            r#"
        SELECT id, user_id, container_name, created_at
        FROM user_containers
        WHERE user_id = $1
        "#
        )
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    /// Finds a container by its unique name (UUID).
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    /// * `container_name` - The unique name (UUID) of the container.
    ///
    /// # Returns
    /// A `Result` containing an optional `UserContainer` or a `sqlx::Error` if the query fails.
    pub async fn find_container_by_name(
        pool: &PgPool,
        container_name: &Uuid,
    ) -> Result<Option<UserContainer>, Error> {
        sqlx::query_as::<_, UserContainer>(
            r#"
        SELECT id, user_id, container_name, created_at
        FROM user_containers
        WHERE container_name = $1
        "#
        )
            .bind(container_name)
            .fetch_optional(pool)
            .await
    }

    /// Stores a new container in the `user_containers` table.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    /// * `new_container` - A `NewContainer` instance containing the details of the new container.
    ///
    /// # Returns
    /// A `Result` containing the created `UserContainer` or a `sqlx::Error` if the query fails.
    pub async fn store_user_container(
        pool: &PgPool,
        new_container: NewContainer,
    ) -> Result<UserContainer, Error> {
        let container = sqlx::query_as::<_, UserContainer>(
            r#"
            INSERT INTO user_containers (user_id, container_name, challenge_id, category_id)
            VALUES ($1, $2, $3, $4)
            RETURNING id, user_id, container_name, challenge_id, category_id, created_at
            "#,
        )
            .bind(new_container.user_id)
            .bind(new_container.container_name)
            .bind(new_container.challenge_id)
            .bind(new_container.category_id)
            .fetch_one(pool)
            .await?;
        Ok(container)
    }
}
