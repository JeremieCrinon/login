pub mod login;
pub mod users;

use axum::{Router, http::{HeaderValue, header::{AUTHORIZATION, CONTENT_TYPE}}};
use jwt_simple::prelude::HS256Key;
use crate::helpers::users::create_jwt_key;
use crate::handlers::api::not_found;
use crate::db::connection::connect;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer, AllowOrigin};
use tera::Tera;
use sea_orm::DatabaseConnection;
use crate::translator::translator::Translator;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub key: Arc<HS256Key>,
    pub tera: Arc<Tera>,
    pub translator: Translator,
}

pub async fn create_router() -> Router {
    // Prepare the app_state, that will contain objects that will be needed in handlers.
    let db = connect().await.expect("Failed to connect to DB"); // Get the database connection to put it in the app_state (you pass this when executing a DB query)
    let key = create_jwt_key(); // Get the JWT key to put it in the app_state, it is needed to create JWT tokens
    let tera = Tera::new("templates/**/*").expect("Failed to load tera templates"); // Instantiate a tera instance to use the email templates (it isn't really optimized to instantiate it each time you wanna send an email) to put it in the app_state
    let tera = Arc::new(tera);
    let translator = Translator::new(); // Create a translator instance that will read and parse the JSON translations files (we instantiate it at app startup to read the translation files just once and not each time we need it)
    let state = AppState { db, key, tera, translator };

    // Get the allowed origins from the .env
    let allowed_origins = std::env::var("ALLOWED_ORIGINS") 
    .unwrap_or_else(|_| "".to_string())
    .split(',')
    .filter_map(|origin| HeaderValue::from_str(origin.trim()).ok())
    .collect::<Vec<HeaderValue>>();

    // Configuring CORS with the allowed origins
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(allowed_origins)) 
        .allow_methods(Any)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]); 

    // Creating the router
    return Router::new()
        .merge(login::login_routes(state.clone())) // Merging the login routes router
        .merge(users::users_routes(state.clone())) // Merging the users gestion routes router
        .fallback(not_found) // 404 route
        .with_state(state)
        .layer(cors); // Add the CORS layer we created above
}