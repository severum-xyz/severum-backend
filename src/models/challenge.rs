use crate::schema::challenges;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = challenges)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Challenge {
    pub id: i32,
    pub category_id: i32,
    pub name: String,
    pub difficulty: String,
    pub description: String,
    pub hint: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = challenges)]
pub struct NewChallenge<'a> {
    pub category_id: i32,
    pub name: &'a str,
    pub difficulty: &'a str,
    pub description: &'a str,
    pub hint: Option<&'a str>,
}