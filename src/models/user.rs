use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub pseudo: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub verified: Option<bool>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub pseudo: &'a str,
    pub password_hash: &'a str,
}
