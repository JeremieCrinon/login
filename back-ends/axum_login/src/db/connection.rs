use sea_orm::{Database, DbConn, DbErr};
use sea_orm_migration::MigratorTrait;
use std::env;
use crate::db::Migrator;
use log::info;

/// This function is called at the app's startup, it connects to the DB and returns it.
pub async fn connect() -> Result<DbConn, DbErr> {
    info!("Connecting to database...");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&database_url).await?;

    Migrator::up(&db, None).await?;
    Ok(db)
}