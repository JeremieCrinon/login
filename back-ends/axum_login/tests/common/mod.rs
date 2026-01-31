use axum_login::routes;
use axum::Router;
use sea_orm::{ActiveValue, Database, DatabaseConnection, EntityTrait};
use sea_orm_migration::MigratorTrait;
use axum_login::db::Migrator;
use axum_login::middlewares::auth::Role;
use axum_login::entities::{user, prelude::User};
use axum_login::helpers::users::hash_passwd;

pub async fn setup_app() -> (Router, DatabaseConnection) {
    dotenvy::from_filename(".env.test").ok();
    
    let db = Database::connect("sqlite::memory:").await.expect("Couldn't connect to sqlite in memory");
    Migrator::up(&db, None).await.expect("Could not execute migration on sqlite test db");

    let app: Router = routes::create_router(Some(db.clone())).await;
    (app, db)
}

pub async fn create_test_user(db: DatabaseConnection, roles: Vec<Role>, email: Option<String>) -> user::Model {
    let email = match email {
        Some(e) => e,
        None => "test@mail.com".to_string()
    };
    let password = "Admin12345@".to_string();
    let hashed_passwd = hash_passwd(password).await.expect("Failed to hash the user password");

    let string_roles: Vec<String> = roles.iter().map(|r| r.as_str().to_string()).collect();
    let json_roles = serde_json::to_value(string_roles).expect("Error converting the roles to json string");

    let new_user = user::ActiveModel {
        email: ActiveValue::set(email),
        password: ActiveValue::set(hashed_passwd),
        roles: ActiveValue::set(json_roles),
        ..Default::default()
    };

    User::insert(new_user).exec_with_returning(&db).await.expect("Error inserting the user into db.")
}
