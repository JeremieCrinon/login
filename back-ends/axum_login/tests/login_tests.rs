mod common;
use axum::{
    Router, body::Body, http::{Request, StatusCode}
};
use tower::ServiceExt;

#[tokio::test]
async fn test_login_with_right_credentials() {
    let app: Router = common::setup_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")   
                .uri("/login")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "email": "email@mail.com",
                        "password": "Admin12345@"
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
