use axum::{
    extract::{Json, State}, http::StatusCode, response::{IntoResponse, Json as AxumJson, Response}, 
    Extension
};
use serde::Deserialize;
use sea_orm::*;
use crate::routes::AppState;
use crate::entities::{prelude::*, *, user::Model as UserModel};
use log::error;

#[derive(Deserialize)]
pub struct PostRoute {
    message: String,
}

pub async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub async fn test_route(State(state): State<AppState>) -> Result<impl IntoResponse, Response> {
    let db = &state.db;

    let users: Vec<user::Model> = User::find().all(db).await.map_err(|e| {
        error!("Database error : {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;

    Ok(Json(users))
}

pub async fn post_message(Json(payload): Json<PostRoute>) -> impl IntoResponse {
    let response = format!("Received message : {}", payload.message);
    AxumJson(response)
}

pub async fn get_user(Extension(user): Extension<Option<UserModel>>) -> impl IntoResponse {
    format!("Bonjour {}", user.unwrap().email)
}

pub async fn not_found() -> impl IntoResponse {
    StatusCode::NOT_FOUND.into_response()
}