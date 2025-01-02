use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub pseudo: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub verified: Option<bool>,
}

pub struct NewUser<'a> {
    pub email: &'a str,
    pub pseudo: &'a str,
    pub password_hash: &'a str,
}

impl<'a> NewUser<'a> {
    pub async fn insert(&self, pool: &sqlx::PgPool) -> Result<i32, sqlx::Error> {
        let row: (i32,) = sqlx::query_as(
            r#"
            INSERT INTO users (email, pseudo, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id
            "#
        )
            .bind(self.email)
            .bind(self.pseudo)
            .bind(self.password_hash)
            .fetch_one(pool)
            .await?;

        Ok(row.0)
    }
}
