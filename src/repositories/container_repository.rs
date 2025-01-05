use diesel::RunQueryDsl;
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::container::UserContainer;

pub struct ContainerRepository;

impl ContainerRepository {

    pub async fn get_all_containers(pool: &PgPool) -> Result<Vec<UserContainer>, sqlx::Error> {
        sqlx::query_as::<_, UserContainer>(
            r#"
        SELECT id, user_id, container_name, created_at
        FROM user_containers
        "#
        )
            .fetch_all(pool)
            .await
    }

    pub async fn find_container_by_user_id(
        pool: &PgPool,
        user_id: i32,
    ) -> Result<Vec<UserContainer>, sqlx::Error> {
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

    pub async fn find_container_by_name(
        pool: &PgPool,
        container_name: &Uuid,
    ) -> Result<Option<UserContainer>, sqlx::Error> {
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

    pub async fn insert_user_container(
        pool: &PgPool,
        user_id: i32,
        container_name: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
        r#"
        INSERT INTO user_containers (user_id, container_name)
        VALUES ($1, $2)
        "#
    )
            .bind(user_id)
            .bind(container_name)
            .execute(pool)
            .await?;
        Ok(())
    }

}
