use sqlx::{PgPool, Error};
use crate::models::user::{NewUser, User};

pub struct UserRepository;

impl UserRepository {
    pub async fn insert_new_user(pool: &PgPool, new_user: &NewUser<'_>) -> Result<i32, Error> {
        let row: (i32,) = sqlx::query_as(
            r#"
        INSERT INTO users (email, pseudo, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id
        "#
        )
            .bind(new_user.email)
            .bind(new_user.pseudo)
            .bind(new_user.password_hash)
            .fetch_one(pool)
            .await?;

        Ok(row.0)
    }

    pub async fn find_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, Error> {
        sqlx::query_as::<_, User>(
            r#"
        SELECT id, email, pseudo, password_hash, created_at, verified
        FROM users
        WHERE email = $1
        "#
        )
            .bind(email)
            .fetch_optional(pool)
            .await
    }

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

    pub async fn pseudo_exists(pool: &PgPool, pseudo: &str) -> Result<bool, Error> {
        let exists: (bool,) = sqlx::query_as(
            r#"
        SELECT EXISTS (
            SELECT 1
            FROM users
            WHERE pseudo = $1
        )
        "#
        )
            .bind(pseudo)
            .fetch_one(pool)
            .await?;

        Ok(exists.0)
    }

}