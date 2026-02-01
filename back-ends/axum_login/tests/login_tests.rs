mod common;
use axum::{
    body::Body, http::{Request, StatusCode}
};
use axum_login::middlewares::auth::Role;
use tower::ServiceExt;
use common::{setup_app, create_test_user, create_user_and_get_jwt};

#[tokio::test]
async fn test_login_with_wrong_email() {
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
                        "email": "unknown@mail.com",
                        "password": "Admin12345@"
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_login_with_wrong_password() {
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
                        "email": "email@mail.com",
                        "password": "Admin12345"
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

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
async fn test_user_infos_with_wrong_token() {
    let (app, db) = setup_app().await;
    let (_, _) = create_user_and_get_jwt(db, vec![Role::User], None).await; // Still call the function even if we don't use the result to have the same context as with the right token

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/user-infos")
                .header("content-type", "application/json")
                .header("authorization", "Bearer aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
                .body(Body::from(""))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
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

#[tokio::test]
async fn test_new_account_with_bad_password() {
    let (app, db) = setup_app().await;
    let token = create_user_and_get_jwt(db, vec![Role::NewAccount], None).await.1;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/modify-new-account/en")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(
                    r#"{
                        "new_email": "test@mail.com",
                        "new_password": "NotSecureEnough"
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_new_account_with_existing_email() {
    let (app, db) = setup_app().await;
    let token = create_user_and_get_jwt(db, vec![Role::NewAccount], None).await.1;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/modify-new-account/en")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(
                    r#"{
                        "new_email": "email@mail.com",
                        "new_password": "SecureEnough1@"
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_new_account_with_right_values_same_email() {
    let (app, db) = setup_app().await;
    let token = create_user_and_get_jwt(db, vec![Role::NewAccount], None).await.1;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/modify-new-account/en")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(
                    r#"{
                        "new_email": "test@mail.com",
                        "new_password": "SecureEnough1@"
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_new_account_with_right_values_other_email() {
    let (app, db) = setup_app().await;
    let token = create_user_and_get_jwt(db, vec![Role::NewAccount], None).await.1;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/modify-new-account/en")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(
                    r#"{
                        "new_email": "newemail@mail.com",
                        "new_password": "SecureEnough1@"
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}

// Tests for verify-email route
#[tokio::test]
async fn test_verify_email_with_wrong_code() {
    let (app, db) = setup_app().await;
    let token = create_user_and_get_jwt(db, vec![Role::UnverifiedEmail], None).await.1;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/verify-email")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(
                    r#"{
                        "code": "aaaaaa"
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_verify_email_with_right_code() {
    let (app, db) = setup_app().await;
    let token = create_user_and_get_jwt(db, vec![Role::UnverifiedEmail], None).await.1;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/verify-email")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(
                    r#"{
                        "code": "code"
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_forgot_password_with_non_existing_email() {
    let (app, db) = setup_app().await;
    let _ = create_test_user(db, vec![], None).await;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/forgot-password/en")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "email": "inexistant@mail.com"
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_forgot_password_with_existing_email() {
    let (app, db) = setup_app().await;
    let _ = create_test_user(db, vec![], None).await;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/forgot-password/en")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "email": "test@mail.com"
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_reset_password_with_wrong_code() {
    let (app, db) = setup_app().await;
    let _ = create_test_user(db, vec![], None).await;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/reset-password")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "code": "wrongcode",
                        "new_password": "SecureEnough1@"
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_reset_password_with_right_code() {
    let (app, db) = setup_app().await;
    let _ = create_test_user(db, vec![], None).await;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/reset-password")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "code": "code",
                        "new_password": "SecureEnough1@"
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_edit_email_unverified_email_no_password() {
    let (app, db) = setup_app().await;
    let token = create_user_and_get_jwt(db, vec![Role::UnverifiedEmail], None).await.1;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/edit-email/en")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(
                    r#"{
                        "new_email": "test@mail.com",
                        "password": ""
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_edit_email_verified_email_no_password() {
    let (app, db) = setup_app().await;
    let token = create_user_and_get_jwt(db, vec![], None).await.1;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/edit-email/en")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(
                    r#"{
                        "new_email": "test@mail.com",
                        "password": ""
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_edit_email_verified_email_wrong_password() {
    let (app, db) = setup_app().await;
    let token = create_user_and_get_jwt(db, vec![], None).await.1;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/edit-email/en")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(
                    r#"{
                        "new_email": "test@mail.com",
                        "password": "wrongpass"
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_edit_email_verified_email_right_password() {
    let (app, db) = setup_app().await;
    let token = create_user_and_get_jwt(db, vec![], None).await.1;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/edit-email/en")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(
                    r#"{
                        "new_email": "test@mail.com",
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
async fn test_edit_password_wrong_current_password() {
    let (app, db) = setup_app().await;
    let token = create_user_and_get_jwt(db, vec![], None).await.1;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/edit-password")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(
                    r#"{
                        "current_password": "wrongpass",
                        "new_password": "SecureEnough1@"
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_edit_password_wrong_new_password() {
    let (app, db) = setup_app().await;
    let token = create_user_and_get_jwt(db, vec![], None).await.1;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/edit-password")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(
                    r#"{
                        "current_password": "Admin12345@",
                        "new_password": "NotSecureEnough"
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_edit_password_right_passwords() {
    let (app, db) = setup_app().await;
    let token = create_user_and_get_jwt(db, vec![], None).await.1;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/edit-password")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(
                    r#"{
                        "current_password": "Admin12345@",
                        "new_password": "SecureEnough1@"
                    }"#
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}
