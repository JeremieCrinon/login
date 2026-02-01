mod common;
use axum::{
    body::Body, http::{Request, StatusCode}
};
use axum_login::middlewares::auth::Role;
use tower::ServiceExt;
use common::{setup_app, create_test_user, create_user_and_get_jwt};

#[tokio::test]
async fn test_user_creation_with_incorrect_email() {
    let (app, db) = setup_app().await;
    let token = create_user_and_get_jwt(db, vec![Role::EditUsers], None).await.1;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users/new/en")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(
                    r#"{
                        "email": "email",
                        "roles": ["admin"]
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_user_creation_with_already_existing_email() {
    let (app, db) = setup_app().await;
    let token = create_user_and_get_jwt(db, vec![Role::EditUsers], None).await.1;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users/new/en")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(
                    r#"{
                        "email": "test@mail.com",
                        "roles": ["admin"]
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_user_creation_with_right_values() {
    let (app, db) = setup_app().await;
    let token = create_user_and_get_jwt(db, vec![Role::Admin], None).await.1;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users/new/en")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(
                    r#"{
                        "email": "user@mail.com",
                        "roles": ["admin"]
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_user_deletion() {
    let (app, db) = setup_app().await;
    let token = create_user_and_get_jwt(db.clone(), vec![Role::Admin], None).await.1;
    let user = create_test_user(db, vec![], Some("delete@mail.com".to_string())).await;
    let id: i32 = user.id;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/users/{}", id))
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
async fn test_user_email_editing_existing_email() {
    let (app, db) = setup_app().await;
    let token = create_user_and_get_jwt(db.clone(), vec![Role::Admin], None).await.1;
    let user = create_test_user(db, vec![], Some("editemail@mail.com".to_string())).await;
    let id: i32 = user.id;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/users/{}/email/en", id))
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(
                    r#"{
                        "email": "test@mail.com"
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_user_email_editing_right_values() {
    let (app, db) = setup_app().await;
    let token = create_user_and_get_jwt(db.clone(), vec![Role::Admin], None).await.1;
    let user = create_test_user(db, vec![], Some("editemail@mail.com".to_string())).await;
    let id: i32 = user.id;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/users/{}/email/en", id))
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(
                    r#"{
                        "email": "unique@mail.com"
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_user_roles_editing() {
    let (app, db) = setup_app().await;
    let token = create_user_and_get_jwt(db.clone(), vec![Role::Admin], None).await.1;
    let user = create_test_user(db, vec![], Some("editemail@mail.com".to_string())).await;
    let id: i32 = user.id;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/users/{}/roles", id))
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(
                    r#"{
                        "roles": ["user"]
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}
