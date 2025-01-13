use sqlx::FromRow;

/// Represents a category in the system.
///
/// This struct maps to the `categories` table and stores details about a category.
#[derive(Debug, FromRow)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

/// Represents a new category to be added to the system.
///
/// Used for inserting a new category into the database.
pub struct NewCategory {
    pub name: String,
}
