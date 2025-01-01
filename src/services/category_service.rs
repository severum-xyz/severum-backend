use diesel::PgConnection;
use diesel::result::Error;
use crate::models::category::{Category, NewCategory};
use crate::repositories::category_repository::CategoryRepository;

pub struct CategoryService;

impl CategoryService {
    pub async fn find_or_create_category(conn: &mut PgConnection, category_name: &str) -> Result<i32, Error> {
        let category = CategoryRepository::find_category_by_name(conn, category_name).await?;

        match category {
            Some(category) => Ok(category.id),
            None => {
                let new_category = NewCategory { name: category_name };
                CategoryRepository::insert_category(conn, &new_category).await?;

                let category = CategoryRepository::find_category_by_name(conn, category_name).await?.unwrap();
                Ok(category.id)
            }
        }
    }

    pub fn get_all_categories(conn: &mut PgConnection) -> Result<Vec<Category>, Error> {
        CategoryRepository::get_all_categories(conn)
    }
}