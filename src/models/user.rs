use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub verified: Option<bool>,
    pub role_id: i32,
}

pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password_hash: String,
}