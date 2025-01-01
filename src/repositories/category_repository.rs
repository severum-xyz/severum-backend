use diesel::prelude::*;
use diesel::result::Error;
use log::info;
use crate::models::category::{Category, NewCategory};
use crate::schema::categories;

pub struct CategoryRepository;

impl CategoryRepository {
    pub async fn insert_category(conn: &mut PgConnection, new_category: &NewCategory<'_>) -> Result<(), Error> {
        info!("Creating new category: {}", new_category.name);
        diesel::insert_into(categories::table)
            .values(new_category)
            .execute(conn)?;
        info!("Category created successfully: {}", new_category.name);
        Ok(())
    }

    pub async fn find_category_by_name(conn: &mut PgConnection, category_name: &str) -> Result<Option<Category>, Error> {
        categories::table
            .filter(categories::name.eq(category_name))
            .first::<Category>(conn)
            .optional()
    }

    pub fn get_all_categories(conn: &mut PgConnection) -> Result<Vec<Category>, Error> {
        categories::table.load::<Category>(conn)
    }
}