pub mod api;
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
    let db = connect().await.expect("Failed to connect to DB");
    let key = create_jwt_key();
    let tera = Tera::new("templates/**/*").expect("Failed to load tera templates");
    let tera = Arc::new(tera);
    let translator = Translator::new();
    let state = AppState { db, key, tera, translator };

    let allowed_origins = std::env::var("ALLOWED_ORIGINS")
    .unwrap_or_else(|_| "".to_string())
    .split(',')
    .filter_map(|origin| HeaderValue::from_str(origin.trim()).ok())
    .collect::<Vec<HeaderValue>>();

    // To configure CORS.
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(allowed_origins)) 
        .allow_methods(Any)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    return Router::new()
        .merge(api::api_routes(state.clone()))
        .merge(login::login_routes(state.clone()))
        .merge(users::users_routes(state.clone()))
        .fallback(not_found)
        .with_state(state)
        .layer(cors);
}