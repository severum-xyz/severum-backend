use sqlx::FromRow;
use chrono::NaiveDateTime;

/// Represents a user in the system.
///
/// This struct maps to the `users` table and stores user account details.
#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub verified: Option<bool>,
    pub role_id: i32,
}

/// Represents a new user to be created in the system.
///
/// Used for inserting user details during registration.
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password_hash: String,
}
