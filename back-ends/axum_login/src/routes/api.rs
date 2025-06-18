use axum::{middleware, routing::{get, post}, Router};
use crate::handlers::api::{hello_world, test_route, post_message, get_user};
use crate::middlewares::auth::{auth, Role};
use crate::routes::AppState;

pub fn api_routes(state: AppState) -> Router<AppState> {
    let admin_state = state.clone();
    let edit_state = state.clone();
    let user_state = state.clone();
    let new_account_state = state.clone();
    let transfer_user_state = state.clone();

    return Router::new()
        .route("/hello-world", get(hello_world))
        .route("/admin", get(test_route).route_layer(middleware::from_fn(move |req, next| {auth(req, next, admin_state.clone(), Role::Admin, false)})))
        .route("/edit-user", get(test_route).route_layer(middleware::from_fn(move |req, next| {auth(req, next, edit_state.clone(), Role::EditUsers, false)})))
        .route("/user", get(test_route).route_layer(middleware::from_fn(move |req, next| {auth(req, next, user_state.clone(), Role::User, false)})))
        .route("/new-account", get(test_route).route_layer(middleware::from_fn(move |req, next| {auth(req, next, new_account_state.clone(), Role::NewAccount, false)})))
        .route("/transfer-user", get(get_user).route_layer(middleware::from_fn(move |req, next| {auth(req, next, transfer_user_state.clone(), Role::User, true)})))
        .route("/message", post(post_message))
}