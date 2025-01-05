use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

pub struct NewCategory {
    pub name: String,
}