use axum::{
    extract::{State, Path}, http::StatusCode, response::{IntoResponse, Response}, Json
};
use sea_orm::{sqlx::types::chrono::Utc, *};
use tera::Context;
use crate::routes::AppState;
use crate::entities::{prelude::*, *};
use log::error;
use validator::Validate;
use serde::{Deserialize, Serialize};
use crate::helpers::{
    users::{validate_roles, hash_passwd, send_email_verification},
    mail::send_mail,
    templates::get_email_context
};
use rand::{distr::Alphanumeric, Rng};
use crate::middlewares::auth::Role;
use std::env;
use lettre::{message::{header::{self, ContentType}, MultiPart, SinglePart}, Message};

#[derive(Serialize)]
struct ListRolesResponse {
    roles: Vec<&'static str>,
}

// This list all the avaible roles so the front-end can display a list of the roles when creating
// or editing a user.
pub async fn list_roles () -> Result<impl IntoResponse, Response> {
    let roles = Role::iter()
    .filter(|r| *r != Role::None && *r != Role::NewAccount && *r != Role::UnverifiedEmail) // We
        // filter the NewAccount and UnverifiedEmail, as we don't want to send them
    .map(|r| r.as_str())
    .collect::<Vec<_>>();

    Ok(Json(ListRolesResponse { roles }))
}

#[derive(Serialize)]
struct ListUsersResponse {
    users: Vec<user::ListUser>,
}

/// Handler for the get route /users. Returns every user with less details than the read_user handler (for the route /user/id).
pub async fn list_users (State(state): State<AppState>) -> Result<impl IntoResponse, Response> {
    let db = state.db;

    let users: Vec<user::Model> = User::find().all(&db).await.map_err(|e| {
        error!("Error fetching all the users in DB : {}", e);

        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;

    let users: Vec<user::ListUser> = users.into_iter().map(user::ListUser::from).collect();

    let response = ListUsersResponse {
       users: users,
    };

    Ok(Json(response))
}

/// Handler for the /users/id get route. It returns a user with his ID with more details that the list_users handler (for the route /users).
pub async fn read_user (State(state): State<AppState>, Path(user_id): Path<i32>) -> Result<impl IntoResponse, Response> {
    let db = state.db;

    let user: Option<user::Model> = User::find_by_id(user_id).one(&db).await.map_err(|e| {
        error!("Error finding a user by id : {}", e);

        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;

    if user.is_none() {
        return Err(StatusCode::NOT_FOUND.into_response());
    }

    let user: user::GetUser = user::GetUser::from(user.unwrap());

    Ok(Json(user))
}

#[derive(Deserialize, Debug, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    email: String,
    #[validate(custom(function = "validate_roles"))]
    roles: Vec<String>,
}

/// Handler for the /users new route. It takes an email and list of roles in the body, validates everything, creates the user in DB, and sends him an email with the new credentials.
pub async fn create_user (State(state): State<AppState>, Path(locale): Path<String>, Json(payload): Json<CreateUserRequest>) -> Result<impl IntoResponse, Response> {
    // Returning a validate error if there is any
    match payload.validate() {
        Ok(_) => (),
        Err(e) => return Err((StatusCode::BAD_REQUEST, format!("{}", e)).into_response()),
    }

    let db = state.db;
    let tera = state.tera;
    let translator = state.translator;
    let mail_sender_address = env::var("MAIL_SENDER_ADRESS").expect("MAIL_SENDER_ADRESS must be set");

    // Verifying if another user already has the email sent (to prevent duplicates)
    let existing_user: Option<user::Model> = user::Entity::find()
    .filter(user::Column::Email.eq(&payload.email))
    .one(&db)
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

    // Generating a random password
    let password: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    // Hashing the password
    let hashed_password = hash_passwd(password.clone()).await.map_err(|e| {
        error!("Error with the hashed password function in the modify_new_account handler : {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    })?;

    let mut roles = payload.roles;

    if !roles.contains(&Role::NewAccount.as_str().to_string()) { // We add the NewAccount role if
        // the front-end hasn't already sent it.
        roles.push(Role::NewAccount.as_str().to_string());
    }

    // Transforming the roles back to JSON
    let json_roles = serde_json::to_value(roles)
    .map_err(|e| {
        error!("Failed to convert roles Vec<String> into JSON: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;

    // Create the new user
    let new_user = user::ActiveModel {
        email: ActiveValue::set(payload.email.clone()),
        password: ActiveValue::set(hashed_password),
        roles: ActiveValue::set(json_roles),
        created_at: ActiveValue::Set(Utc::now().naive_utc()),
        updated_at: ActiveValue::Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    // Get the tera templates for the mail (one for the HTML, one for the fallback text)
    let translations = translator.get_translation(&locale);

    let mut context: Context = get_email_context(&translations);
    context.insert("email", payload.email.as_str());
    context.insert("password", password.as_str());

    let html_body = tera.render("create_account/email.html.tera", &context).map_err(|e| {
        error!("Error getting the tera template : {}", e);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;
    let text_body = tera.render("create_account/email.txt.tera", &context).map_err(|e| {
        error!("Error gitting the tera template : {}", e);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;

    let email_subject: String = translations.get("create_account_email_subject").ok_or_else(|| {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?
    .to_string();

    // Create the mail
    let email = Message::builder()
        .from(mail_sender_address.parse().unwrap())
        .to(payload.email.parse().unwrap())
        .subject(email_subject)
        .header(ContentType::TEXT_PLAIN)
        // .body(String::from(format!("Here is your password : {}", password)))
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
        error!("Error sending the invite email for a new user : {}", e);

        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;

    // Put the user in DB after the mail has been sent in case of errors in the mail sending, we don't want to have a user in DB that hasn't received the mail to create their account, while it's less of a problem if a user as received an email while their account hasn't actually been created. And, they are much less chances of having an error putting a user in DB than when sending an email.
    User::insert(new_user).exec(&db).await.map_err(|e| {
        error!("Error inserting a new user in DB : {}", e);

        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;
    
    Ok(("User created successfully.").into_response())
}

/// The handler for the /users/id delete route. It deletes the user in DB.
pub async fn delete_user (State(state): State<AppState>, Path(user_id): Path<i32>) -> Result<impl IntoResponse, Response> {
    let db = state.db;

    if user_id == 1 {
        return Err((StatusCode::BAD_REQUEST, "You cannot delete the user with id = 1").into_response());
    }

    let user = user::ActiveModel {
        id: ActiveValue::Set(user_id),
        ..Default::default()
    };
    user.delete(&db).await.map_err(|e| {
        error!("Error deleting the user in DB : {}", e);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;

    Ok(("User deleted successfully.").into_response())
}

#[derive(Deserialize, Debug, Validate)]
pub struct EditUserRolesRequest {
    #[validate(custom(function = "validate_roles"))]
    roles: Vec<String>,
}

/// The handler for the /users/id/roles edit route. It takes the new roles and edits the user in DB with them.
pub async fn edit_user_roles (State(state): State<AppState>, Path(user_id): Path<i32>, Json(payload): Json<EditUserRolesRequest>) -> Result<impl IntoResponse, Response> {
    // Returning a validate error if there is any
    match payload.validate() {
        Ok(_) => (),
        Err(e) => return Err((StatusCode::BAD_REQUEST, format!("{}", e)).into_response()),
    }

    // If it is the first user (created automatically by the migration) we refuse to edit his roles, as he is a special user that should not be edited like this (to always have at least one admin).
    if user_id == 1 {
        return Err((StatusCode::BAD_REQUEST, "You cannot edit the roles of the user with id = 1").into_response());
    }
    
    let db = state.db;

    // Verifying if the user exists
    let existing_user = User::find_by_id(user_id).one(&db).await.map_err(|e| {
        error!("Failed to query user by id: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;
    
    if existing_user.is_none() {
        return Err(StatusCode::NOT_FOUND.into_response());
    }

    // Transforming the roles to JSON
    let json_roles = serde_json::to_value(payload.roles)
    .map_err(|e| {
        error!("Failed to convert roles Vec<String> into JSON: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;

    let user = user::ActiveModel {
        id: ActiveValue::Set(user_id),
        roles: ActiveValue::Set(json_roles),
        ..Default::default()
    };
    user.update(&db).await.map_err(|e| {
        error!("Error updating user in DB : {}", e);

        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;
    
    Ok(("User roles updated successfully.").into_response())
}

#[derive(Deserialize, Debug, Validate)]
pub struct EditUserEmailRequest {
    #[validate(email)]
    email: String,
}

/// The handler for the /users/id/email edit route. It takes the new email and edits the user in DB with it. It the calls the helper send_email_verification.
pub async fn edit_user_email (State(state): State<AppState>, Path((user_id, locale)): Path<(i32, String)>, Json(payload): Json<EditUserEmailRequest>) -> Result<impl IntoResponse, Response> {
    // Returning a validate error if there is any
    match payload.validate() {
        Ok(_) => (),
        Err(e) => return Err((StatusCode::BAD_REQUEST, format!("{}", e)).into_response()),
    }

    let db = &state.db;

    // Verifying if another user already has the new email sent (to prevent duplicates)
    let existing_user: Option<user::Model> = user::Entity::find()
    .filter(user::Column::Email.eq(&payload.email))
    .filter(user::Column::Id.ne(user_id)) // So we don't select the user we want to edit
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

    // Verifying if the user exists
    let existing_user = User::find_by_id(user_id).one(db).await.map_err(|e| {
        error!("Failed to query user by id: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;
    
    if existing_user.is_none() {
        return Err(StatusCode::NOT_FOUND.into_response());
    }

    // Updating the user in DB
    let updated_user = user::ActiveModel {
        id: ActiveValue::Set(user_id),
        email: ActiveValue::Set(payload.email),
        ..Default::default()
    };
    updated_user.update(db).await.map_err(|e| {
        error!("Error updating user in DB : {}", e);

        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?;

    let user = User::find_by_id(user_id).one(db).await.map_err(|e| {
        error!("Failed to query user by id: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })?.ok_or_else(|| {
        error!("Cannot find the newly updated user in edit_user_email");
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    })
    .unwrap();

    // Calling the send_email_verification function
    send_email_verification(user, db.clone(), state, locale).await.map_err(|e| {
        error!("Error sending the user's email verification code : {}", e);

        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    })?;

    Ok("User email updated successfully.")
}
