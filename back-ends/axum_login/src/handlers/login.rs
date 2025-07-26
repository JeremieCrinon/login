use std::env;

use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    extract::{Json, State, Path},
    Extension,
};
use serde::{Serialize, Deserialize};
use password_worker::PasswordWorker;
use jwt_simple::prelude::*;
use sea_orm::{sqlx::types::chrono::Utc, *};
use crate::routes::AppState;
use crate::entities::{prelude::*, *};
use log::error;
use validator::Validate;
use crate::middlewares::auth::Role;
use crate::helpers::users::{send_email_verification, hash_passwd, validate_password};
use rand::{distr::Alphanumeric, Rng};
use lettre::{message::{header::{self, ContentType}, MultiPart, SinglePart}, Message};
use crate::helpers::mail::send_mail;
use crate::helpers::templates::get_email_context;
use tera::Context;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserClaims {
    pub id: i32,
}

#[derive(Serialize)]
struct UserInfosResponse {
    result: bool,
    roles: Vec<String>,
    user_mail: String,
}

/// Handler for the /user-infos route. It returns a result that can be true if the user has the role admin, or false if he dont, the user's roles, and the user's email.
pub async fn user_infos(Extension(user): Extension<Option<user::Model>>) -> Result<impl IntoResponse, Response> { // Get the user from the auth middleware 
    let user = user.unwrap();
     
    let roles_array = user.roles.as_array().cloned().unwrap_or_default();

    let roles: Vec<String> = roles_array
        .into_iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect();

    let is_admin = roles.iter().any(|role| role == Role::Admin.as_str());

    let response = UserInfosResponse {
        result: is_admin,
        roles,
        user_mail: user.email,
    };

    Ok(axum::Json(response))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

/// Handler for the /login route. Takes an email and password, verifies that they are correct, and returns a JWT if it is.
pub async fn login(State(state): State<AppState>, Json(payload): Json<LoginRequest>) -> Result<impl IntoResponse, Response> {
    let db = &state.db; // Get the DB from the AppState
    let key = &state.key; // Get the JWT key from the AppState
    let max_threads = 4;
    let password_worker = PasswordWorker::new_bcrypt(max_threads).map_err(|e| {
        error!("Could not create password worker : {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;

    let app_name = std::env::var("APP_NAME").expect("APP_NAME is missing"); // Get the app name from the .env

    let user: Option<user::Model> = User::find() // Make a DB query to get the user
        .filter(user::Column::Email.eq(payload.email)) // Search only the one that has the email sent in the request
        .one(db)
        .await
        .map_err(|e| {
            error!("Database error : {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?;

    let user = match user {
        Some(u) => u, // If we find a user, we continue
        None => return Err((axum::http::StatusCode::BAD_REQUEST, "Email or password incorrect").into_response()), // Else, we return an error
    };

    let is_valid = password_worker
        .verify(payload.password, user.password) // Verify the password sent in the request with the password worker
        .await
        .map_err(|e| {
            error!("Password verification failed: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?;

    if !is_valid { // If the password worker returned false, it means the password sent isn't correct
        return Err((axum::http::StatusCode::BAD_REQUEST, "Email or password incorrect").into_response());
    }

    let mut claims = Claims::with_custom_claims( // We prepare our  claims (what the JWT will contain)
        UserClaims {
            id: user.id // We put the user ID in the JWT, so we will be able to get it in our middleware, then get them in DB to know their roles and if they still exists. We could put the roles directly in the JWT, so we won't have to do a DB request each time we wanna verify permissions, but it will mean that if we delete the user or change roles, it will not update immediatly.
        },
        Duration::from_secs(3600 * 24 * 30), // The token is valid for 30 days
    );

    claims.issuer = Some(app_name.to_string()); // We put the issuer value to us

    let token = key 
    .authenticate(claims)
    .map(Json)
    .map_err(|e| {
        error!("Token generation error : {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?; // Create the token with the claims we made

    let response = LoginResponse {
        token: token.to_string(),
    };

    Ok(axum::Json(response))
}



#[derive(Deserialize, Debug, Validate)]
pub struct ModifyNewAccountRequest {
    #[validate(email)]
    new_email: String,
    #[validate(custom(function = "validate_password"))]
    new_password: String,
}

/// Handler for the /modify-new-account route. Takes a new_email (that can be the same has the user already has) and a new_password, verifies everything, updates the user in DB and calls the helper send_email_verification.
pub async fn modify_new_account (State(state): State<AppState>, Extension(user): Extension<Option<user::Model>>, Path(locale): Path<String>, Json(payload): Json<ModifyNewAccountRequest>) -> Result<impl IntoResponse, Response> {
    // Returning a validate error if there is any
    match payload.validate() {
        Ok(_) => (),
        Err(e) => return Err((StatusCode::BAD_REQUEST, format!("{}", e)).into_response()),
    }

    let user = user.unwrap();
    let db = &state.db;

    // Verifying if another user already has the new email sent (to prevent duplicates)
    let existing_user: Option<user::Model> = user::Entity::find()
    .filter(user::Column::Email.eq(&payload.new_email))
    .filter(user::Column::Id.ne(user.id)) // So we don't select the user himself if he doesn't change his email
    .one(db)
    .await
    .map_err(|e| {
        error!("Database error while checking for duplicate email: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;

    if existing_user.is_some() {
        return Err((
            StatusCode::CONFLICT,
            "Email is already used by another account."
        ).into_response());
    }

    // Hashing the password
    let hashed_password = hash_passwd(payload.new_password).await.map_err(|e| {
        error!("Error with the hashed password function in the modify_new_account handler : {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    })?;

    // Removing the new_account from the user's roles
    let mut roles: Vec<String> = serde_json::from_value(user.roles.clone()).unwrap_or_default();

    roles.retain(|role| role != Role::NewAccount.as_str());

    // Transforming the roles back to JSON
    let json_roles = serde_json::to_value(roles)
    .map_err(|e| {
        error!("Failed to convert roles Vec<String> into JSON: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;

    // Create a updated user
    let new_user = user::ActiveModel {
        id: ActiveValue::Set(user.id),
        email: ActiveValue::Set(payload.new_email),
        password: ActiveValue::Set(hashed_password),
        roles: ActiveValue::Set(json_roles),
        updated_at: ActiveValue::Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    // Update the user in the DB
    new_user.update(db).await.map_err(|e| {
        error!("Database error while updating a new user : {}", e);

        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    })?;

    // Get the user again to send him to the send_email_verification function that will add him the unverified_email role, assign him a verification code, and send him an email with this verification code
    let user = user::Entity::find_by_id(user.id)
    .one(db)
    .await
    .map_err(|e| {
        error!("Error fetching the updated user : {}", e);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?
    .ok_or_else(|| {
        error!("Cannot find the newly updated user in modify_new_account");
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })
    .unwrap();

    // Calling the send_email_verification function
    send_email_verification(user, db.clone(), state, locale).await.map_err(|e| {
        error!("Error sending the user's email verification code : {}", e);

        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    })?;
    
    Ok(("User updated successfully, you might want to force the user to login again, but this is not required by the back-end.").into_response())
}

#[derive(Deserialize, Debug, Validate)]
pub struct VerifyEmailRequest {
    code: String
}

/// This is the handler for the /verify-email route. Takes the code that has been sent to the user by email, verifies that it is valid, and removes the user's unverified_email role.
pub async fn verify_email (State(state): State<AppState>, Extension(user): Extension<Option<user::Model>>, Json(payload): Json<VerifyEmailRequest>) -> Result<impl IntoResponse, Response> {
    let user = user.unwrap();
    let db = state.db;

    if user.email_verification_code.as_deref() != Some(&payload.code) {
        return Err((
            StatusCode::BAD_REQUEST,
            "The code you sent isn't the right one."
        ).into_response());
    }

    // Removing the unverified_email from the user's roles
    let mut roles: Vec<String> = serde_json::from_value(user.roles.clone()).unwrap_or_default();

    roles.retain(|role| role != Role::UnverifiedEmail.as_str());

    // Transforming the roles back to JSON
    let json_roles = serde_json::to_value(roles)
    .map_err(|e| {
        error!("Failed to convert roles Vec<String> into JSON: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;

    // Create a updated user
    let new_user = user::ActiveModel {
        id: ActiveValue::Set(user.id),
        email_verification_code: ActiveValue::Set(None),
        roles: ActiveValue::Set(json_roles),
        ..Default::default()
    };

    // Update the user in the DB
    new_user.update(&db).await.map_err(|e| {
        error!("Database error while updating a user with unverified email : {}", e);

        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    })?;

    Ok(("Email verified successfully.").into_response())
}

#[derive(Deserialize, Debug, Validate)]
pub struct ForgotPasswordRequest {
    email: String,
}

/// This is the handler for the /forgot-password route. It takes the email of the user, gets the user, assign him a random code in DB, and sends a link with this code in email.
pub async fn forgot_password (State(state): State<AppState>, Path(locale): Path<String>, Json(payload): Json<ForgotPasswordRequest>) -> Result<impl IntoResponse, Response> {
    let db = state.db;
    let tera = state.tera;
    let translator = state.translator;
    let mail_sender_address = env::var("MAIL_SENDER_ADRESS").expect("MAIL_SENDER_ADRESS must be set");
    let web_front_end = env::var("APP_MAIN_FRONT_END").expect("APP_MAIN_FRONT_END must be set");

    let user: Option<user::Model> = User::find()
        .filter(user::Column::Email.eq(payload.email))
        .one(&db)
        .await
        .map_err(|e| {
            error!("Error getting the user by email in forgot_password handler : {}", e);

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?;
    
    if user.is_none() {
        return Ok(("Password reset link sent to the email you provided if it exists.").into_response()); // We return the exact same thing in case the email corresponds to no user, because we don't want the client to know if the email corresponds to a user or not for security reasons.
    }

    let user = user.unwrap();

    // Generating a random password reset code
    let code: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    
    // Edit the user to have the verification code we generated
    let new_user = user::ActiveModel {
        id: ActiveValue::Set(user.id),
        password_reset_code: ActiveValue::set(Some(code.clone())),
        ..Default::default()
    };

    // Update the user in the DB
    new_user.update(&db).await.map_err(|e| {
        error!("Error updating the user in DB in forgot_password handler: {}", e);

        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;

    let link = format!("{}/forgot-password/{}", web_front_end, code); // We make the link that will be in the email with the random code we made above

    // Get the tera templates for the mail (one for the HTML, one for the fallback text)
    let translations = translator.get_translation(&locale);

    let mut context: Context = get_email_context(&translations);
    context.insert("link", &link);

    let html_body = tera.render("forgot_password/email.html.tera", &context).map_err(|e| {
        error!("Error getting the tera template : {}", e);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;
    let text_body = tera.render("forgot_password/email.txt.tera", &context).map_err(|e| {
        error!("Error gitting the tera template : {}", e);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;

    let email_subject: String = translations.get("forgot_password_email_subject").ok_or_else(|| {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?
    .to_string();

    // We can send the email with the verification code after updating the user in DB, if the sending does not work, it is not a big deal. Unlike in the send_email_verification handler, where the roles are edited.
    let email = Message::builder()
        .from(mail_sender_address.parse().unwrap())
        .to(user.email.parse().unwrap())
        .subject(email_subject)
        .header(ContentType::TEXT_PLAIN)
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
        error!("Error sending the email with the password reset link : {}", e);

        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;

    Ok(("Password reset link sent to the email you provided if it exists.").into_response())
}

#[derive(Deserialize, Debug, Validate)]
pub struct ResetPasswordRequest {
    code: String,
    #[validate(custom(function = "validate_password"))]
    new_password: String,
}

/// This is the handler for the /reset-password route. It takes a code that has been sent to the user in an URL in an email, gets the user with this code, and changes the password with the new password if it gets a user, if it does not get a user with this code, it means that the code isn't valid. It then removes the code from the user.
pub async fn reset_password (State(state): State<AppState>, Json(payload): Json<ResetPasswordRequest>) -> Result<impl IntoResponse, Response> {
    // Returning a validate error if there is any
    match payload.validate() {
        Ok(_) => (),
        Err(e) => return Err((StatusCode::BAD_REQUEST, format!("{}", e)).into_response()),
    }
    
    let db = state.db;
    
    let user: Option<user::Model> = User::find()
        .filter(user::Column::PasswordResetCode.eq(payload.code))
        .one(&db)
        .await
        .map_err(|e| {
            error!("Error getting the user by the code in reset_password handler : {}", e);

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?;
    
    if user.is_none() {
        return Err((
            StatusCode::UNAUTHORIZED,
            "The code in the link isn't the right one."
        ).into_response());
    }

    let user = user.unwrap();

    // Hashing the password
    let hashed_password = hash_passwd(payload.new_password).await.map_err(|e| {
        error!("Error with the hashed password function in the reset_password handler : {}", e);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;

    let new_user = user::ActiveModel {
        id: ActiveValue::Set(user.id),
        password: ActiveValue::set(hashed_password),
        password_reset_code: ActiveValue::set(None),
        ..Default::default()
    };

    // Update the user in the DB
    new_user.update(&db).await.map_err(|e| {
        error!("Error updating the user in DB in forgot_password handler: {}", e);

        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;

    Ok(("Password reset successfully, you can now log in with it.").into_response())
}

#[derive(Deserialize, Debug, Validate)]
pub struct EditEmailRequest {
    #[validate(email)]
    new_email: String,
    password: String, // Facultative because if the user hasn't yet validated they're email adress, we do not ask for the password
}

/// The handler for the /edit-email route. This ables already logged in users to edit their email. It takes a new email and the user's current password (unless the user hasn't yet verified his email adress, in that case, we do not require their current password to simplify the procedure)
pub async fn edit_email (State(state): State<AppState>, Extension(user): Extension<Option<user::Model>>, Path(locale): Path<String>, Json(payload): Json<EditEmailRequest>) -> Result<impl IntoResponse, Response> {
    // Returning a validate error if there is any
    match payload.validate() {
        Ok(_) => (),
        Err(e) => return Err((StatusCode::BAD_REQUEST, format!("{}", e)).into_response()),
    }

    let db = &state.db;
    let user = user.unwrap();

    let user_roles = user.roles.as_array().expect("The users roles are not parsable for some reason. Error occured in edit_email handler in the login.rs.");
    
    if !user_roles.iter().any(|r| r.as_str() == Some(Role::UnverifiedEmail.as_str())) { // We need to verify the password
        if payload.password.is_empty() {
            return Err(
                (
                    StatusCode::BAD_REQUEST,
                    "You need to send your current password to continue."
                ).into_response()
            );
        }

        let password_worker = PasswordWorker::new_bcrypt(4).map_err(|e| {
            error!("Could not create password worker : {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?;
        
        let is_valid = password_worker
            .verify(payload.password, user.password)
            .await
            .map_err(|e| {
                error!("Password verification failed: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            })?;

        if !is_valid {
            return Err((StatusCode::UNAUTHORIZED, "The password sent is incorrect.").into_response());
        }
    }

    // Verify that the new_email isn't already taken
    let existing_user: Option<user::Model> = user::Entity::find()
        .filter(user::Column::Email.eq(&payload.new_email))
        .filter(user::Column::Id.ne(user.id)) // So we don't select the user himself if he doesn't change his email
        .one(db)
        .await
        .map_err(|e| {
            error!("Database error while checking for duplicate email: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?;

    if existing_user.is_some() {
        return Err((
            StatusCode::CONFLICT,
            "Email is already used by another account."
        ).into_response());
    }

    // We've done our verifications, now, we modify the user
    let new_user = user::ActiveModel {
        id: ActiveValue::Set(user.id),
        email: ActiveValue::Set(payload.new_email),
        ..Default::default()
    };

    // Update the user in the DB
    new_user.update(db).await.map_err(|e| {
        error!("Error updating the user in DB in edit_email handler: {}", e);

        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;

    // Get the user again to send him to the send_email_verification function that will add him the unverified_email role, assign him a verification code, and send him an email with this verification code
    let user = user::Entity::find_by_id(user.id)
    .one(db)
    .await
    .map_err(|e| {
        error!("Error fetching the updated user : {}", e);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?
    .ok_or_else(|| {
        error!("Cannot find the newly updated user in modify_new_account");
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })
    .unwrap();

    // Calling the send_email_verification function
    send_email_verification(user, db.clone(), state, locale).await.map_err(|e| {
        error!("Error sending the user's email verification code : {}", e);

        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    })?;

    Ok(("Email successfully edited, the user now has the unverified_email role, you have to disconnect them.").into_response())
}

#[derive(Deserialize, Debug, Validate)]
pub struct EditPasswordRequest {
    current_password: String,
    #[validate(custom(function = "validate_password"))]
    new_password: String,
}

/// This is the handler for the /edit-password route. This ables already logged in users, that haven't forgot their password, to edit it with a new one. It takes the user's current password (even tough the user is already logged in, we still ask for the password to add one more security), and the new password.
pub async fn edit_password (State(state): State<AppState>, Extension(user): Extension<Option<user::Model>>, Json(payload): Json<EditPasswordRequest>) -> Result<impl IntoResponse, Response> {
    // Returning a validate error if there is any
    match payload.validate() {
        Ok(_) => (),
        Err(e) => return Err((StatusCode::BAD_REQUEST, format!("{}", e)).into_response()),
    }

    let db = state.db;
    let user = user.unwrap();

    
    if payload.current_password.is_empty() {
        return Err(
            (
                StatusCode::BAD_REQUEST,
                "You need to send your current password to continue."
            ).into_response()
        );
    }

    let password_worker = PasswordWorker::new_bcrypt(4).map_err(|e| {
        error!("Could not create password worker : {}", e);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;
    
    let is_valid = password_worker
        .verify(payload.current_password, user.password)
        .await
        .map_err(|e| {
            error!("Password verification failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })?;

    if !is_valid {
        return Err((StatusCode::UNAUTHORIZED, "The current password sent is incorrect.").into_response());
    }

    // We've done our verifications, now, we modify the user

    // Hashing the password
    let hashed_password = hash_passwd(payload.new_password).await.map_err(|e| {
        error!("Error with the hashed password function in the edit_password handler : {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    })?;


    let new_user = user::ActiveModel {
        id: ActiveValue::Set(user.id),
        password: ActiveValue::Set(hashed_password),
        ..Default::default()
    };

    // Update the user in the DB
    new_user.update(&db).await.map_err(|e| {
        error!("Error updating the user in DB in edit_email handler: {}", e);

        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;

    Ok(("Password successfully edited.").into_response())
}
