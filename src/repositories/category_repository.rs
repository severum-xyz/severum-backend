use sqlx::{PgPool, Error};
use crate::models::category::Category;

pub struct CategoryRepository;

impl CategoryRepository {
    pub async fn get_all_categories(pool: &PgPool) -> Result<Vec<Category>, Error> {
        sqlx::query_as::<_, Category>(
            r#"
            SELECT id, name
            FROM categories
            "#
        )
            .fetch_all(pool)
            .await
    }

    pub async fn find_category_by_name(pool: &PgPool, category_name: &str) -> Result<Option<Category>, Error> {
        sqlx::query_as::<_, Category>(
            r#"
            SELECT id, name
            FROM categories
            WHERE name = $1
            "#
        )
            .bind(category_name)
            .fetch_optional(pool)
            .await
    }

    pub async fn insert_category(pool: &PgPool, category_name: &str) -> Result<i32, Error> {
        let row: (i32,) = sqlx::query_as(
            r#"
            INSERT INTO categories (name)
            VALUES ($1)
            RETURNING id
            "#
        )
            .bind(category_name)
            .fetch_one(pool)
            .await?;

        Ok(row.0)
    }
}
