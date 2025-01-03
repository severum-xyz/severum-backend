use sqlx::FromRow;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize, Debug, FromRow)]
pub struct Challenge {
    pub id: i32,
    pub category_id: i32,
    pub name: String,
    pub difficulty: String,
    pub description: String,
    pub hint: Option<String>,
    pub created_at: NaiveDateTime,
}

pub struct NewChallenge<'a> {
    pub category_id: i32,
    pub name: &'a str,
    pub difficulty: &'a str,
    pub description: &'a str,
    pub hint: Option<&'a str>,
}

impl<'a> NewChallenge<'a> {
    pub async fn insert(&self, pool: &sqlx::PgPool) -> Result<i32, sqlx::Error> {
        let row: (i32,) = sqlx::query_as(
            r#"
            INSERT INTO challenges (category_id, name, difficulty, description, hint)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
            "#
        )
            .bind(self.category_id)
            .bind(self.name)
            .bind(self.difficulty)
            .bind(self.description)
            .bind(self.hint)
            .fetch_one(pool)
            .await?;

        Ok(row.0)
    }
}
