use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{extract::State, Json};
use serde::Deserialize;
use serde_json::json;
use tracing::error;
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

use crate::{db::models::users::User, error::BlogError, utils::jwt::Claims, AppState};

#[derive(Deserialize, ToSchema)]
pub struct CreateUser {
    username: String,
    nickname: String,
    email: String,
    password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateUserInfo {
    nickname: String,
}

// #[derive(Deserialize)]
// pub struct UpdateUserPass {
//     password: String,
// }

// #[derive(Deserialize)]
// pub struct UpdateUserEmail {
//     email: String,
// }

#[derive(OpenApi)]
#[openapi(
    paths(create_user, update_user_info),
    components(schemas(CreateUser, UpdateUserInfo))
)]
pub(super) struct UserDoc;

#[utoipa::path(
    post,
    path = "/user",
    request_body = CreateUser,
    responses(
        (status = 200, description = "User created successfully"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error"),
    ),
)]
pub async fn create_user(
    state: State<AppState>,
    Json(new_user): Json<CreateUser>,
) -> Result<String, BlogError> {
    let mut conn = state.pool.get_owned().await.map_err(|e| {
        error!("{}", e);
        BlogError::InternalServerError
    })?;

    let username = &new_user.username;
    let user = User::find_user_by_name(username, &mut conn)
        .await
        .map_err(|e| {
            error!("{}", e);
            BlogError::InternalServerError
        })?;
    if user.is_some() {
        return Err(BlogError::BadRequest("username already exists".to_string()));
    }

    let email = &new_user.email;
    let user = User::find_user_by_email(email, &mut conn)
        .await
        .map_err(|e| {
            error!("{}", e);
            BlogError::InternalServerError
        })?;
    if user.is_some() {
        return Err(BlogError::BadRequest("email already exists".to_string()));
    }

    let nickname = &new_user.nickname;
    let password = &new_user.password;

    // hash password
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let password_hash = &argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| {
            error!("{}", e);
            BlogError::InternalServerError
        })?
        .to_string();

    User::created_user(username, nickname, email, password_hash, &mut conn)
        .await
        .map_err(|e| {
            error!("{}", e);
            BlogError::InternalServerError
        })?;

    Ok(json!({"ok": "user successfully created"}).to_string())
}

#[utoipa::path(
    put,
    path = "/user",
    request_body = UpdateUserInfo,
    responses(
        (status = 200, description = "User updated successfully"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error"),
    ),
    security(
        ("jwt_token" = [])
    )
)]
pub async fn update_user_info(
    state: State<AppState>,
    claims: Claims,
    Json(update_info): Json<UpdateUserInfo>,
) -> Result<String, BlogError> {
    let mut conn = state.pool.get_owned().await.map_err(|e| {
        error!("{}", e);
        BlogError::InternalServerError
    })?;

    let id = Uuid::parse_str(&claims.user_id)?;
    let nickname = update_info.nickname;
    User::update_user_info(&id, &nickname, &mut conn)
        .await
        .map_err(|e| {
            error!("{}", e);
            BlogError::InternalServerError
        })?;

    Ok(json!({"ok": "user successfully updated"}).to_string())
}

// pub async fn update_user_email(
//     state: State<AppState>,
//     claims: Claims,
//     Json(update_email): Json<UpdateUserEmail>,
// ) -> Result<String, BlogError> {
//     let mut conn = state.pool.get_owned().await.map_err(|e| {
//         error!("{}", e);
//         BlogError::InternalServerError
//     })?;

//     todo!();
// }

// pub async fn update_user_passwd(
//     state: State<AppState>,
//     claims: Claims,
//     Json(update_passwd): Json<UpdateUserPass>,
// ) -> Result<String, BlogError> {
//     let mut conn = state.pool.get_owned().await.map_err(|e| {
//         error!("{}", e);
//         BlogError::InternalServerError
//     })?;
//     todo!()
// }
