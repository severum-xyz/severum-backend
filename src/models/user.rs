use sqlx::FromRow;
use chrono::NaiveDateTime;

/// Represents a user in the system.
///
/// This struct maps to the `users` table in the database and contains information
/// about the user's account, such as their email, username, and account status.
#[allow(dead_code)]
#[derive(FromRow)]
pub struct User {
    /// The unique ID of the user.
    pub id: i32,

    /// The email address of the user.
    pub email: String,

    /// The username of the user.
    pub username: String,

    /// The hashed password of the user.
    pub password_hash: String,

    /// The timestamp of when the user account was created.
    pub created_at: NaiveDateTime,

    /// Whether the user's email address has been verified.
    pub verified: Option<bool>,

    /// The ID of the role assigned to the user.
    pub role_id: i32,
}

/// Represents a new user to be created in the system.
///
/// This struct is used when registering a new user, prior to assigning them an ID
/// or other system-generated values.
pub struct NewUser {
    /// The email address of the new user.
    pub email: String,

    /// The username of the new user.
    pub username: String,

    /// The hashed password of the new user.
    pub password_hash: String,
}
