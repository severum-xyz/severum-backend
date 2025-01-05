use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize)]
pub struct UserContainer {
    pub id: i32,
    pub user_id: i32,
    pub challenge_id: i32,
    pub category_id: i32,
    pub container_name: Uuid,
    pub created_at: NaiveDateTime,
}

pub struct NewContainer {
    pub user_id: i32,
    pub challenge_id: i32,
    pub category_id: i32,
    pub container_name: Uuid,
}