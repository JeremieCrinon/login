mod common;
use axum::{
    body::Body, http::{Request, StatusCode}
};
use axum_login::middlewares::auth::Role;
use tower::ServiceExt;
use common::{setup_app, create_user_and_get_jwt};


#[tokio::test]
async fn test_protected_route_without_token() {
    let (app, _db) = setup_app().await;
    
    // Call a route that requires the edit_users role without sending a JWT
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/edit_users")
                .header("content-type", "application/json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_protected_route_without_right_role() {
    let (app, db) = setup_app().await;
    let token = create_user_and_get_jwt(db, vec![Role::User], None).await.1;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/edit_users")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_protected_route_with_new_account_role() {
    let (app, db) = setup_app().await;
    let token = create_user_and_get_jwt(db, vec![Role::NewAccount, Role::EditUsers], None).await.1;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/edit_users")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_protected_route_with_right_role() {
    let (app, db) = setup_app().await;
    let token = create_user_and_get_jwt(db, vec![Role::EditUsers], None).await.1;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/edit_users")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_protected_route_with_admin_role() {
    let (app, db) = setup_app().await;
    let token = create_user_and_get_jwt(db, vec![Role::Admin], None).await.1;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/edit_users")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}
