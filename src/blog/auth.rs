use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::error;
use utoipa::{OpenApi, ToSchema};

use crate::{db::models::users::User, error::BlogError, utils::jwt::encode_jwt, AppState};

#[derive(Deserialize, ToSchema)]
pub struct Login {
    #[schema(example = "tsuiio")]
    identity: String,
    #[schema(example = "nekonya")]
    password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginReturn {
    token: String,
}

#[derive(OpenApi)]
#[openapi(paths(login), components(schemas(LoginReturn, Login)))]
pub struct AuthDoc;

#[utoipa::path(
    post,
    path = "/login",
    request_body = Login,
    responses(
        (status = 200, description = "Login successfully", body = LoginReturn),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn login(
    state: State<AppState>,
    Json(login): Json<Login>,
) -> Result<Json<LoginReturn>, BlogError> {
    let mut conn = state.pool.get_owned().await.map_err(|e| {
        error!("{}", e);
        BlogError::InternalServerError
    })?;

    if login.identity.is_empty() || login.password.is_empty() {
        return Err(BlogError::WrongCredentials);
    }

    let password = login.password;
    let user = User::login_by_email_or_name(&login.identity, &mut conn)
        .await
        .map_err(|e| {
            error!("{}", e);
            BlogError::InternalServerError
        })?
        .ok_or(BlogError::Unauthorized(String::from(
            "invalid password or username",
        )))?;

    let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|e| {
        error!("{}", e);
        BlogError::InternalServerError
    })?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|e| match e {
            argon2::password_hash::Error::Password => {
                BlogError::Unauthorized(String::from("invalid password or username"))
            }
            _ => BlogError::InternalServerError,
        })?;

    let token = encode_jwt(&user.id)?;

    Ok(Json(LoginReturn { token }))
}
