use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use jwt_simple::prelude::*;
use sea_orm::{EntityTrait, EnumIter};
use crate::routes::AppState;
use crate::entities::prelude::*;
use log::error;
use crate::handlers::login::UserClaims;
use crate::entities::user::Model as UserModel;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter)]
pub enum Role {
    Admin,
    User,
    NewAccount,
    UnverifiedEmail,
    EditUsers,
    None,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Admin => "admin",
            Role::User => "user",
            Role::NewAccount => "new_account",
            Role::UnverifiedEmail => "unverified_email",
            Role::EditUsers => "edit_users",
            Role::None => "none",
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "admin" => Some(Role::Admin),
            "user" => Some(Role::User),
            "new_account" => Some(Role::NewAccount),
            "unverified_email" => Some(Role::UnverifiedEmail),
            "edit_users" => Some(Role::EditUsers),
            //"none" => Some(Role::None) // Never uncomment this line, I put it here so future devs will know not to do this mistake. The role None should only be used to tell we do not want to verify a role. And in that case, we do not need to translate it from a string. If uncomented, this role could be given to users.
            _ => None,
        }
    }
}

/// A middleware that verifies that the user corresponding to the JWT sent in the request headers has the required role.
pub async fn auth(
    mut request: Request,
    next: Next,
    state: AppState,
    required_role: Role,
    inject_user: bool,
) -> Result<Response, StatusCode> {
    let headers = request.headers();

    let token = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let db = &state.db;
    let key = &state.key;

    let claims = key
        .verify_token::<UserClaims>(token, None)
        .map_err(|_| {
            StatusCode::UNAUTHORIZED
        })?;

    let user = User::find_by_id(claims.custom.id)
        .one(db)
        .await
        .map_err(|e| {
            error!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    token_is_valid(user.clone(), required_role).await?;

    if inject_user {
        request.extensions_mut().insert(user);
    }

    Ok(next.run(request).await)
}

async fn token_is_valid(user: Option<UserModel>, required_role: Role) -> Result<(), StatusCode> {

    if user.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let user = user.unwrap();

    if let Some(roles_array) = user.roles.as_array() {
        let has_new_account = roles_array.iter().any(|val| val.as_str() == Some(Role::NewAccount.as_str()));
        let has_unverified_email = roles_array.iter().any(|val| val.as_str() == Some(Role::UnverifiedEmail.as_str()));

        if required_role == Role::None {
            return Ok(()) // We do not require anything, the middleware is just used to get the user.
        }

        if (has_new_account && required_role != Role::NewAccount) || (has_unverified_email && (required_role != Role::UnverifiedEmail && required_role!= Role::NewAccount)) {
            return Err(StatusCode::UNAUTHORIZED); // If the user has a new account or an unverified email, and he tries to access another route than that, we return an UNAUTHORIZED status code
        }

        if required_role == Role::User {
            return Ok(()) // No need to ge further, we don't require any role
        }

        let has_admin = roles_array.iter().any(|val| val.as_str() == Some(Role::Admin.as_str()));
        if has_admin && required_role != Role::NewAccount && required_role != Role::UnverifiedEmail {
            return Ok(()); // The user is admin, he can go, unless he wants to go to a route for the new accounts or account with unverified email, in that case, we will verify that he has the role corresponding to that
        }

        let has_role = roles_array.iter().any(|val| val.as_str() == Some(required_role.as_str()));

        if !has_role {
            return Err(StatusCode::UNAUTHORIZED);
        }
    } else {
        error!("Roles field is not an array: {:?}", user.roles);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(())
}   