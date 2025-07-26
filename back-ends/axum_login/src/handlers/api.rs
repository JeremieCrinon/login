use axum::{
    http::StatusCode, response::{IntoResponse}
};

// This is the handler for all routes that are not listed
pub async fn not_found() -> impl IntoResponse {
    StatusCode::NOT_FOUND.into_response()
}
