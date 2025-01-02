use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

pub struct NewCategory<'a> {
    pub name: &'a str,
}

impl<'a> NewCategory<'a> {
    pub async fn insert(&self, pool: &sqlx::PgPool) -> Result<i32, sqlx::Error> {
        let row: (i32,) = sqlx::query_as(
            r#"
            INSERT INTO categories (name)
            VALUES ($1)
            RETURNING id
            "#
        )
            .bind(self.name)
            .fetch_one(pool)
            .await?;

        Ok(row.0)
    }
}
