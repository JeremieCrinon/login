mod common;
use axum::{
    body::Body, http::{Request, StatusCode}
};
use axum_login::middlewares::auth::Role;
use tower::ServiceExt;
use common::{setup_app, create_test_user, create_user_and_get_jwt};

#[tokio::test]
async fn test_login_with_right_credentials() {
    let (app, db) = setup_app().await;
    let _ = create_test_user(db, vec![Role::User], None).await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")   
                .uri("/login")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "email": "test@mail.com",
                        "password": "Admin12345@"
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_user_infos_with_right_token() {
    let (app, db) = setup_app().await;
    let (_, token) = create_user_and_get_jwt(db, vec![Role::User], None).await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/user-infos")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(""))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}
