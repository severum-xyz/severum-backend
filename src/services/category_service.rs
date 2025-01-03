use sqlx::PgPool;
use crate::models::category::Category;
use crate::repositories::category_repository::CategoryRepository;

pub struct CategoryService;

impl CategoryService {
    pub async fn get_all_categories(pool: &PgPool) -> Result<Vec<Category>, sqlx::Error> {
        CategoryRepository::get_all_categories(pool).await
    }

    pub async fn find_or_create_category(pool: &PgPool, category_name: &str) -> Result<Category, sqlx::Error> {
        match CategoryRepository::find_category_by_name(pool, category_name).await? {
            Some(category) => Ok(category),
            None => {
                let category_id = CategoryRepository::insert_category(pool, category_name).await?;
                Ok(Category {
                    id: category_id,
                    name: category_name.to_string(),
                })
            }
        }
    }
}
