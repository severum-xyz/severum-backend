use std::env;
use diesel::{Connection, PgConnection};

pub async fn get_db_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    PgConnection::establish(&database_url)
        .expect("Failed to connect to the database")
}