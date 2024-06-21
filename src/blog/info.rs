use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::error;
use utoipa::{IntoParams, OpenApi, ToSchema};
use uuid::Uuid;

use crate::{db::models::info::Info, error::BlogError, utils::jwt::Claims, AppState};

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct CreateInfo {
    #[schema(example = "tsuiio's blog")]
    bio: String,
    #[schema(example = "blog")]
    title: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ReturnInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Uuid>,
    bio: String,
    title: String,
}

#[derive(Deserialize, ToSchema)]

pub struct UpdateInfo {
    #[schema(example = "tsuiio's blog")]
    bio: String,
    #[schema(example = "blog")]
    title: String,
}

#[derive(OpenApi)]
#[openapi(
    paths(get_info, create_info, update_info),
    components(schemas(ReturnInfo, UpdateInfo, CreateInfo))
)]
pub struct InfoDoc;

#[utoipa::path(
    post,
    path = "/info",
    request_body = CreateInfo,
    responses(
        (status = 200, description = "Info created successfully"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt_token" = [])
    )
)]
pub async fn create_info(
    state: State<AppState>,
    _claims: Claims,
    Json(new_info): Json<CreateInfo>,
) -> Result<String, BlogError> {
    let mut conn = state.pool.get_owned().await.map_err(|e| {
        error!("{}", e);
        BlogError::InternalServerError
    })?;

    let have_info = Info::get_info(&mut conn)
        .await
        .map_err(|e| {
            error!("{}", e);
            BlogError::InternalServerError
        })?
        .is_none();

    if have_info {
        Info::create_info(&new_info.bio, &new_info.title, &mut conn)
            .await
            .map_err(|e| {
                error!("{}", e);
                BlogError::InternalServerError
            })?;

        Ok(json!({ "OK": "create info ok!"}).to_string())
    } else {
        Err(BlogError::BadRequest(String::from("having info!!")))
    }
}

#[utoipa::path(
    put,
    path = "/info",
    responses(
        (status = 200, description = "Info update successfully"),
        (status = 500, description = "Internal server error")
    ),
    request_body = UpdateInfo,
    security(
        ("jwt_token" = [])
    )
)]
pub async fn update_info(
    state: State<AppState>,
    _claims: Claims,
    Json(update_info): Json<UpdateInfo>,
) -> Result<String, BlogError> {
    let mut conn = state.pool.get_owned().await.map_err(|e| {
        error!("{}", e);
        BlogError::InternalServerError
    })?;

    Info::update_info(&update_info.bio, &update_info.title, &mut conn)
        .await
        .map_err(|e| {
            error!("{}", e);
            BlogError::InternalServerError
        })?;

    Ok(json!({ "OK": "update info ok!"}).to_string())
}

#[utoipa::path(
    get,
    path = "/info",
    responses(
        (status = 200, description = "Info retrieved successfully", body = ReturnInfo),
        (status = 404, description = "Info not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_info(state: State<AppState>) -> Result<Json<ReturnInfo>, BlogError> {
    let mut conn = state.pool.get_owned().await.map_err(|e| {
        error!("{}", e);
        BlogError::InternalServerError
    })?;

    let info = Info::get_info(&mut conn).await.map_err(|e| {
        error!("{}", e);
        BlogError::InternalServerError
    })?;

    if let Some(info) = info {
        Ok(Json(ReturnInfo {
            id: None,
            bio: info.bio,
            title: info.title,
        }))
    } else {
        Err(BlogError::NotFound(String::from("Not found info")))
    }
}
