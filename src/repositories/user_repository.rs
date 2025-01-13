use sqlx::{PgPool, Error};
use crate::models::user::{NewUser, User};

/// Repository for managing user-related operations in the database.
pub struct UserRepository;

impl UserRepository {
    /// Inserts a new user into the `users` table.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    /// * `new_user` - A `NewUser` struct containing the user's email, username, and hashed password.
    ///
    /// # Returns
    /// A `Result` containing the ID of the newly created user, or a `sqlx::Error` if the operation fails.
    pub async fn insert_new_user(pool: &PgPool, new_user: NewUser) -> Result<i32, Error> {
        let row: (i32,) = sqlx::query_as(
            r#"
            INSERT INTO users (email, username, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id
            "#
        )
            .bind(new_user.email)
            .bind(new_user.username)
            .bind(new_user.password_hash)
            .fetch_one(pool)
            .await?;

        Ok(row.0)
    }

    /// Finds a user by their username in the `users` table.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    /// * `username` - The username of the user to search for.
    ///
    /// # Returns
    /// A `Result` containing an optional `User` if found, or a `sqlx::Error` if the operation fails.
    pub async fn find_user_by_username(pool: &PgPool, username: &str) -> Result<Option<User>, Error> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, username, password_hash, created_at, verified
            FROM users
            WHERE username = $1
            "#
        )
            .bind(username)
            .fetch_optional(pool)
            .await
    }

    /// Checks if an email address exists in the `users` table.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    /// * `email` - The email address to check for existence.
    ///
    /// # Returns
    /// A `Result` containing a boolean indicating whether the email exists, or a `sqlx::Error` if the operation fails.
    pub async fn email_exists(pool: &PgPool, email: &str) -> Result<bool, Error> {
        let exists: (bool,) = sqlx::query_as(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM users
                WHERE email = $1
            )
            "#
        )
            .bind(email)
            .fetch_one(pool)
            .await?;

        Ok(exists.0)
    }

    /// Checks if a username exists in the `users` table.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    /// * `username` - The username to check for existence.
    ///
    /// # Returns
    /// A `Result` containing a boolean indicating whether the username exists, or a `sqlx::Error` if the operation fails.
    pub async fn username_exists(pool: &PgPool, username: &str) -> Result<bool, Error> {
        let exists: (bool,) = sqlx::query_as(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM users
                WHERE username = $1
            )
            "#
        )
            .bind(username)
            .fetch_one(pool)
            .await?;

        Ok(exists.0)
    }

    /// Retrieves the role ID of a user by their user ID from the `users` table.
    ///
    /// # Arguments
    /// * `pool` - A reference to the database connection pool.
    /// * `user_id` - The ID of the user whose role is being retrieved.
    ///
    /// # Returns
    /// A `Result` containing the user's role ID as an `i32`, or a `sqlx::Error` if the operation fails.
    pub async fn get_user_role(pool: &PgPool, user_id: i32) -> Result<i32, Error> {
        sqlx::query_scalar::<_, i32>(
            r#"
            SELECT role_id
            FROM users
            WHERE id = $1
            "#
        )
            .bind(user_id)
            .fetch_one(pool)
            .await
    }
}
