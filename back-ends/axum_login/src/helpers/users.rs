use crate::entities::*;
use sea_orm::*;
use axum::http::StatusCode;
use crate::middlewares::auth::Role;
use log::error;
use rand::{distr::Alphanumeric, Rng};
use lettre::{message::{header::{self, ContentType}, MultiPart, SinglePart}, Message};
use std::env;
use password_worker::{BcryptConfig, PasswordWorker};
use jwt_simple::prelude::*;
use std::sync::Arc;
use crate::helpers::{mail::send_mail, templates::get_email_context};
use validator::ValidationError;
use crate::routes::AppState;
use tera::Context;

/// This function creates the key for the JWTs, it is called at the app's startup
pub fn create_jwt_key() -> Arc<HS256Key> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET is missing");
    Arc::new(HS256Key::from_bytes(secret.as_bytes()))
}

/// This function is called by handlers, gets a non hashed password and returns an hashed password
pub async fn hash_passwd(password: String) -> Result<String, StatusCode> {
    let cost = 12;
    let max_threads = 4;

    let password_worker = PasswordWorker::new_bcrypt(max_threads).map_err(|e| {
        error!("Error creating the password worker to hash a password : {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let hashed_password = password_worker.hash(password, BcryptConfig { cost }).await.map_err(|e| {
        error!("Error hashing the password : {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(hashed_password)
}

/// This function takes a user, attributes him a random email verification code, and send him an email with it
pub async fn send_email_verification (user: user::Model, db: DatabaseConnection, state: AppState, locale: String) -> Result<(), StatusCode> {
    let mut roles: Vec<String> = serde_json::from_value(user.roles.clone()).unwrap_or_default();
    let mail_sender_address = env::var("MAIL_SENDER_ADRESS").expect("MAIL_SENDER_ADRESS must be set");

    if !roles.contains(&Role::UnverifiedEmail.as_str().to_string()) {
        roles.push(Role::UnverifiedEmail.as_str().to_string());
    }

    let tera = state.tera;
    let translator = state.translator;

    // Transforming the roles back to JSON
    let json_roles = serde_json::to_value(roles)
    .map_err(|e| {
        error!("Failed to convert roles Vec<String> into JSON: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Generating a random email verification code
    let code: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    // Create the updated user that will be updated in DB
    let new_user = user::ActiveModel {
        id: ActiveValue::Set(user.id),
        roles: ActiveValue::Set(json_roles),
        email_verification_code: ActiveValue::Set(Some(code.clone())),
        ..Default::default()
    };

    // Send the mail before updating the user in DB (we prefer having sent the mail without an updated user than having an updated user without a sent email)
    // Get the tera templates for the mail (one for the HTML, one for the fallback text)
    let translations = translator.get_translation(&locale);

    let mut context: Context = get_email_context(&translations);
    context.insert("code", &code);

    let html_body = tera.render("verify_email/email.html.tera", &context).map_err(|e| {
        error!("Error getting the tera template : {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let text_body = tera.render("verify_email/email.txt.tera", &context).map_err(|e| {
        error!("Error gitting the tera template : {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let email_subject: String = translations.get("verify_email_email_subject").ok_or_else(|| {
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .to_string();
    
    let email = Message::builder()
        .from(mail_sender_address.parse().unwrap())
        .to(user.email.parse().unwrap())
        .subject(email_subject)
        .header(ContentType::TEXT_PLAIN)
        // .body(String::from(format!("Here is your code : {}", code.clone())))
        .multipart(
            MultiPart::alternative()
            .singlepart(
                SinglePart::builder()
                .header(header::ContentType::TEXT_PLAIN)
                .body(text_body)   
            )
            .singlepart(
                SinglePart::builder()
                    .header(header::ContentType::TEXT_HTML)
                    .body(html_body)
            )
        )
        .unwrap();

    send_mail(email).await.map_err(|e| {
        error!("Error sending the email with the email verification code : {}", e);

        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Update the user in DB if the mail sending went well
    new_user.update(&db).await.map_err(|e| {
        error!("Database error while updating a user to add the email verification : {}", e);

        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(())
}

/// This function is called by the validator to validate that a password respects the app's requirements
pub fn validate_password(pw: &str) -> Result<(), ValidationError> {
    if pw.len() < 8 {
        return Err(
            ValidationError::new("Password must be at least 8 characters long"));
    }

    if pw.len() > 63 {
        return Err(
            ValidationError::new("Password must be 63 characters long maximum"));
    }

    if !pw.chars().any(|c| c.is_ascii_uppercase()) {
        return Err(ValidationError::new("Password must contain at least one uppercase letter"));
    }
    if !pw.chars().any(|c| c.is_ascii_lowercase()) {
        return Err(ValidationError::new("Password must contain at least one lowercase letter"));
    }
    if !pw.chars().any(|c| c.is_ascii_digit()) {
        return Err(ValidationError::new("Password must contain at least one digit"));
    }

    Ok(())
}

/// This function is called by the validator to validate that a list of roles respects the app's requirements
pub fn validate_roles(roles: &Vec<String>) -> Result<(), ValidationError> {
    for role in roles  {
        if Role::from_str(role).is_none() {
            return Err({
                let mut err = ValidationError::new("invalid_role");
                err.message = Some(format!("The role '{}' does not exist.", role).into());
                err
            });
        }
    }

    Ok(())
}
