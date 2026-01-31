use axum_login::routes;
use axum::Router;
use sea_orm::Database;
use sea_orm_migration::MigratorTrait;
use axum_login::db::Migrator;

pub async fn setup_app() -> Router {
    dotenvy::from_filename(".env.test").ok();
    
    let db = Database::connect("sqlite::memory:").await.expect("Couldn't connect to sqlite in memory");
    Migrator::up(&db, None).await.expect("Could not execute migration on sqlite test db");

    let app: Router = routes::create_router(Some(db)).await;
    app
}
