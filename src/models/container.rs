use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct UserContainer {
    pub id: i32,
    pub user_id: i32,
    pub container_name: Uuid,
    pub created_at: NaiveDateTime,
}
