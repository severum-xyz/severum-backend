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

pub struct NewChallenge {
    pub category_id: i32,
    pub name: String,
    pub difficulty: String,
    pub description: String,
    pub hint: Option<String>,
}