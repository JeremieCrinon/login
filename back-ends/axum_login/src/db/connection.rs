use sea_orm::{Database, DbConn, DbErr};
use sea_orm_migration::MigratorTrait;
use std::env;
use crate::db::Migrator;
use log::info;

/// This function is called at the app's startup, it connects to the DB and returns it.
pub async fn connect() -> Result<DbConn, DbErr> {
    info!("Connecting to database...");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set"); // Get the database url from the .env
    let db = Database::connect(&database_url).await?; // Connect sea-orm to db

    Migrator::up(&db, None).await?; // Execute new migration if there is new ones
    Ok(db) // Return the db connection
}