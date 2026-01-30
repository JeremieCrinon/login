use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use axum_login::routes;

#[tokio::test]
async fn test_login_with_right_credentials() {
    dotenvy::from_filename(".env.test").ok();
    let app = routes::create_router().await;

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
