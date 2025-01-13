use sqlx::FromRow;
use chrono::NaiveDateTime;
use serde::Serialize;

/// Represents a challenge in the system.
///
/// This struct maps to the `challenges` table and includes details such as
/// the category, difficulty, and description of a challenge.
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

/// Represents a new challenge to be created in the system.
///
/// Used for inserting challenge details into the database.
pub struct NewChallenge {
    pub category_id: i32,
    pub name: String,
    pub difficulty: String,
    pub description: String,
    pub hint: Option<String>,
}
