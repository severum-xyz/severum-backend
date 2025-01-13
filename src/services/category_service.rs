use sqlx::PgPool;
use crate::models::category::{Category, NewCategory};
use crate::repositories::category_repository::CategoryRepository;

/// Service for managing categories, including retrieval and creation.
pub struct CategoryService;

impl CategoryService {
    /// Retrieves all categories from the database.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    ///
    /// # Returns
    /// A `Result` containing a vector of `Category` or a `sqlx::Error` if the operation fails.
    pub async fn get_all_categories(pool: &PgPool) -> Result<Vec<Category>, sqlx::Error> {
        CategoryRepository::get_all_categories(pool).await
    }

    /// Finds an existing category by its name or creates a new one if it doesn't exist.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    /// * `category_name` - The name of the category to find or create.
    ///
    /// # Returns
    /// A `Result` containing the `Category` or a `sqlx::Error` if the operation fails.
    pub async fn find_or_create_category(pool: &PgPool, category_name: &str) -> Result<Category, sqlx::Error> {
        let new_category = NewCategory {
            name: category_name.to_string(),
        };

        match CategoryRepository::find_category_by_name(pool, category_name).await? {
            Some(category) => Ok(category),
            None => {
                let category_id = CategoryRepository::insert_category(pool, new_category).await?;
                Ok(Category {
                    id: category_id,
                    name: category_name.to_string(),
                })
            }
        }
    }
}
