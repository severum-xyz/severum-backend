use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use crate::schema::users;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub pseudo: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub verified: bool,
}

#[derive(Insertable, Deserialize, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub pseudo: &'a str,
    pub password_hash: &'a str,
}
