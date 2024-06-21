use axum::{
    extract::{Path, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::error;
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

use crate::{db::models::tags::Tag, error::BlogError};
use crate::{utils::jwt::Claims, AppState};

#[derive(Deserialize, ToSchema)]
pub struct CreateTag {
    #[schema(example = "nekonya")]
    pub content: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateTag {
    #[schema(example = "nekonya")]
    pub content: String,
}

#[derive(Serialize, ToSchema)]
pub struct ReturnTag {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Uuid>,
    content: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Serialize, ToSchema)]
pub struct ReturnTags {
    tags: Vec<ReturnTag>,
}

#[derive(OpenApi)]
#[openapi(
    paths(create_tag, get_tags, delete_tag, update_tag),
    components(schemas(CreateTag, UpdateTag, ReturnTag, ReturnTags))
)]
pub struct TagDoc;

#[utoipa::path(
    post,
    path = "/tag",
    request_body = CreateTag,
    responses(
        (status = 200, description = "Tag created successfully"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt_token" = [])
    )
)]
pub async fn create_tag(
    state: State<AppState>,
    _claims: Claims,
    c_tag: Json<CreateTag>,
) -> Result<String, BlogError> {
    let mut conn = state.pool.get_owned().await.map_err(|e| {
        error!("{}", e);
        BlogError::InternalServerError
    })?;

    Tag::create_tag(&c_tag.content, &mut conn)
        .await
        .map_err(|e| {
            error!("{}", e);
            BlogError::InternalServerError
        })?;

    Ok(json!({"ok": "create tag ok"}).to_string())
}

#[utoipa::path(
    get,
    path = "/tags/{page}",
    params(
        ("page" = u64, Path, description = "List Tags by page")
    ),
    responses(
        (status = 200, description = "Tags retrieved successfully", body = ReturnTags),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_tags(
    Path(page): Path<u64>,
    state: State<AppState>,
) -> Result<Json<ReturnTags>, BlogError> {
    let mut conn = state.pool.get_owned().await.map_err(|e| {
        error!("{}", e);
        BlogError::InternalServerError
    })?;

    if page == 0 {
        return Err(BlogError::BadRequest(String::from("not 0!")));
    }

    let limit = 50;
    let offset = (page - 1) * limit;

    let tags = Tag::get_tags(limit as i64, offset as i64, &mut conn)
        .await
        .map_err(|e| {
            error!("{}", e);
            BlogError::InternalServerError
        })?
        .into_iter()
        .map(|t| ReturnTag {
            id: Some(t.id),
            content: t.content,
            created_at: t.created_at,
            updated_at: t.updated_at,
        })
        .collect();

    Ok(Json(ReturnTags { tags }))
}

#[utoipa::path(
    put,
    path = "/tag/{tag_id}",
    params(
        ("tag_id" = Uuid, Path, description = "Tag id")
    ),
    responses(
        (status = 200, description = "Tag update successfully"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt_token" = [])
    )
)]
pub async fn update_tag(
    state: State<AppState>,
    Path(id): Path<Uuid>,
    Json(u_tag): Json<UpdateTag>,
) -> Result<String, BlogError> {
    let mut conn = state.pool.get_owned().await.map_err(|e| {
        error!("{}", e);
        BlogError::InternalServerError
    })?;

    Tag::update_tag_by_uuid(&id, &u_tag.content, &mut conn)
        .await
        .map_err(|e| {
            error!("{}", e);
            BlogError::InternalServerError
        })?;

    Ok(json!({"ok": "update tag ok"}).to_string())
}

#[utoipa::path(
    delete,
    path = "/tag/{tag_id}",
    params(
        ("tag_id" = Uuid, Path, description = "Tag id")
    ),
    responses(
        (status = 200, description = "Tag delete successfully"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt_token" = [])
    )
)]
pub async fn delete_tag(state: State<AppState>, Path(id): Path<Uuid>) -> Result<String, BlogError> {
    let pool = &state.pool;

    Tag::delete_tag_by_uuid(&id, pool.clone()).await?;

    Ok(json!({"ok": "delete tag ok"}).to_string())
}
