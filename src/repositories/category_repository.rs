use sqlx::{PgPool, Error};
use crate::models::category::Category;

/// Repository for managing categories in the database.
pub struct CategoryRepository;

impl CategoryRepository {
    /// Retrieves all categories from the `categories` table.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    ///
    /// # Returns
    /// A `Result` containing a vector of `Category` or a `sqlx::Error` if the query fails.
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

    /// Finds a category by its name in the `categories` table.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    /// * `category_name` - The name of the category to search for.
    ///
    /// # Returns
    /// A `Result` containing an optional `Category` if the category exists,
    /// or a `sqlx::Error` if the query fails.
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

    /// Inserts a new category into the `categories` table and returns its ID.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    /// * `category_name` - The name of the category to insert.
    ///
    /// # Returns
    /// A `Result` containing the ID of the newly created category,
    /// or a `sqlx::Error` if the query fails.
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