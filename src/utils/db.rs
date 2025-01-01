use std::env;
use diesel::{Connection, PgConnection};
use log::{error, info, warn};

pub async fn get_db_connection() -> Result<PgConnection, ()> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    PgConnection::establish(&database_url).map_err(|e| {
        error!("Failed to connect to the database: {}", e);
        warn!("Try running \"diesel setup\" to create the database and run migrations.\n\n");
    })
}