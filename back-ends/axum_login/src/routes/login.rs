use axum::{Router, routing::{get, post}, middleware};
use crate::handlers::login::{login, modify_new_account, verify_email, user_infos, forgot_password, reset_password, edit_email, edit_password};
use crate::middlewares::auth::{auth, Role};
use crate::routes::AppState;

pub fn login_routes(state: AppState) -> Router<AppState> {
    let new_account_state = state.clone();
    let verify_email_state = state.clone();
    let user_infos_state = state.clone();
    let edit_email_state = state.clone();
    let edit_password_state = state.clone();

    return Router::new()
        .route("/login", post(login))
        .route("/modify-new-account/{locale}", post(modify_new_account).route_layer(middleware::from_fn(move |req, next| {auth(req, next, new_account_state.clone(), Role::NewAccount, true)})))
        .route("/verify-email", post(verify_email).route_layer(middleware::from_fn(move |req, next| {auth(req, next, verify_email_state.clone(), Role::UnverifiedEmail, true)})))
        .route("/user-infos", get(user_infos).route_layer(middleware::from_fn(move |req, next| {auth(req, next, user_infos_state.clone(), Role::None, true)})))
        .route("/forgot-password/{locale}", post(forgot_password))
        .route("/reset-password", post(reset_password))
        .route("/edit-email/{locale}", post(edit_email).route_layer(middleware::from_fn(move |req, next| {auth(req, next, edit_email_state.clone(), Role::None, true)})))
        .route("/edit-password", post(edit_password).route_layer(middleware::from_fn(move |req, next| {auth(req, next, edit_password_state.clone(), Role::User, true)})))
}