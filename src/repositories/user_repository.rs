use diesel::prelude::*;
use diesel::result::Error;
use crate::models::user::{NewUser, User};
use crate::schema::users;

pub fn insert_new_user(conn: &mut PgConnection, new_user: &NewUser) -> Result<(), Error> {
    diesel::insert_into(users::table)
        .values(new_user)
        .execute(conn)?;
    Ok(())
}

pub fn find_user_by_email(conn: &mut PgConnection, email: &str) -> Result<Option<User>, Error> {
    users::table
        .filter(users::email.eq(email))
        .first::<User>(conn)
        .optional()
}

pub fn email_exists(conn: &mut PgConnection, email: &str) -> Result<bool, Error> {
    diesel::select(diesel::dsl::exists(
        users::table.filter(users::email.eq(email))
    ))
        .get_result(conn)
}

pub fn pseudo_exists(conn: &mut PgConnection, pseudo: &str) -> Result<bool, Error> {
    diesel::select(diesel::dsl::exists(
        users::table.filter(users::pseudo.eq(pseudo))
    ))
        .get_result(conn)
}