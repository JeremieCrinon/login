use axum::{Router, routing::{get, post, delete, put}, middleware};
use crate::handlers::users::{list_users, read_user, create_user, delete_user, edit_user_roles, edit_user_email, list_roles};
use crate::middlewares::auth::{auth, Role};
use crate::routes::AppState;

pub fn users_routes(state: AppState) -> Router<AppState> {
    let list_users_state = state.clone();
    let read_user_state = state.clone();
    let create_user_state = state.clone();
    let delete_user_state = state.clone();
    let edit_user_roles_state = state.clone();
    let edit_user_email_state = state.clone();
    let list_roles_state = state.clone();

    return Router::new()
        .route("/users", get(list_users).route_layer(middleware::from_fn(move |req, next| {auth(req, next, list_users_state.clone(), Role::EditUsers, false)})))
        .route("/users/{user_id}", get(read_user).route_layer(middleware::from_fn(move |req, next| {auth(req, next, read_user_state.clone(), Role::EditUsers, false)})))
        .route("/users/new/{locale}", post(create_user).route_layer(middleware::from_fn(move |req, next| {auth(req, next, create_user_state.clone(), Role::EditUsers, false)})))
        .route("/users/{user_id}", delete(delete_user).route_layer(middleware::from_fn(move |req, next| {auth(req, next, delete_user_state.clone(), Role::EditUsers, false)})))
        .route("/users/{user_id}/roles", put(edit_user_roles).route_layer(middleware::from_fn(move |req, next| {auth(req, next, edit_user_roles_state.clone(), Role::EditUsers, false)})))
        .route("/users/{user_id}/email/{locale}", put(edit_user_email).route_layer(middleware::from_fn(move |req, next| {auth(req, next, edit_user_email_state.clone(), Role::EditUsers, false)})))
        .route("/users/list-roles", get(list_roles).route_layer(middleware::from_fn(move |req, next| {auth(req, next, list_roles_state.clone(), Role::EditUsers, false)})))
}