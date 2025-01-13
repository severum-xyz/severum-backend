use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

/// Represents a user's container in the system.
///
/// This struct maps to the `user_containers` table and includes details
/// such as the user ID, challenge, and category associated with the container.
#[derive(Debug, FromRow, Serialize)]
pub struct UserContainer {
    pub id: i32,
    pub user_id: i32,
    pub challenge_id: i32,
    pub category_id: i32,
    pub container_name: Uuid,
    pub created_at: NaiveDateTime,
}

/// Represents a new container to be created in the system.
///
/// Used for inserting container details into the database.
pub struct NewContainer {
    pub user_id: i32,
    pub challenge_id: i32,
    pub category_id: i32,
    pub container_name: Uuid,
}
