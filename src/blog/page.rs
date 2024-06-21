pub mod about;

use about::{CreateAbout, ReturnAbout, UpdateAbout};
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

use crate::{
    db::models::{
        notes::Status,
        pages::{about::AboutPage, Page, PageImpl, PageTi::About},
    },
    error::BlogError,
    utils::jwt::Claims,
    AppState,
};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type")]
pub enum CreatePageTs {
    #[serde(rename = "about")]
    About(CreateAbout),
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type")]
pub enum ReturnPageTs {
    #[serde(rename = "about")]
    About(ReturnAbout),
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type")]
pub enum UpdatePageTs {
    #[serde(rename = "about")]
    About(UpdateAbout),
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReturnPage {
    id: Option<Uuid>,
    page: ReturnPageTs,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Deserialize, ToSchema)]
pub struct CreatePage {
    pub stauts: Status,
    pub subname: String,
    pub comm: bool,
    pub page: CreatePageTs,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdatePage {
    pub status: Status,
    pub subname: String,
    pub comm: bool,
    pub page: UpdatePageTs,
}

#[derive(OpenApi)]
#[openapi(
    paths(create_page, get_page, update_page, delete_page),
    components(schemas(
        CreatePage,
        ReturnPage,
        UpdatePage,
        CreatePageTs,
        ReturnPageTs,
        UpdatePageTs,
        CreateAbout,
        ReturnAbout,
        UpdateAbout,
    ))
)]
pub struct PageDoc;

#[utoipa::path(
    get,
    path = "/page/{short_id}",
    params(
        ("short_id" = String, Path, description = "Page short_id")
    ),
    responses(
        (status = 200, description = "Page retrieved successfully", body = ReturnPage),
        (status = 404, description = "Page not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_page(
    state: State<AppState>,
    Path(short_id): Path<String>,
) -> Result<Json<ReturnPage>, BlogError> {
    let pool = &state.pool;

    let (page_id, created_at, updated_at, page_ti) =
        PageImpl::find_page_by_short_id(&short_id, pool.clone())
            .await
            .map_err(|e| {
                error!("{}", e);
                BlogError::InternalServerError
            })?
            .ok_or_else(|| BlogError::NotFound(String::from("Page not found")))?;

    let page = match page_ti {
        About(about) => ReturnPage {
            id: Some(page_id),
            created_at,
            updated_at,
            page: ReturnPageTs::About(ReturnAbout {
                id: Some(about.id),
                avatar_url: about.avatar_url,
                content: about.content,
            }),
        },
    };

    Ok(Json(page))
}

#[utoipa::path(
    post,
    path = "/page",
    request_body = CreatePage,
    responses(
        (status = 200, description = "Page created successfully"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt_token" = [])
    )
)]
pub async fn create_page(
    state: State<AppState>,
    claims: Claims,
    Json(new_page): Json<CreatePage>,
) -> Result<String, BlogError> {
    let pool = &state.pool;
    let user_id = Uuid::parse_str(&claims.user_id)?;

    match new_page.page {
        CreatePageTs::About(about) => {
            let new_about = AboutPage {
                id: Uuid::new_v4(),
                avatar_url: about.avatar_url,
                content: about.content,
            };
            new_about
                .create_page(
                    &new_page.stauts,
                    &new_page.subname,
                    new_page.comm,
                    &user_id,
                    pool.clone(),
                )
                .await?;

            Ok(json!({ "ok": "create page ok!"}).to_string())
        }
    }
}

#[utoipa::path(
    delete,
    path = "/page/{page_id}",
    params(
        ("page_id" = Uuid, Path, description = "Page short_id")
    ),
    responses(
        (status = 200, description = "Page delete successfully"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt_token" = [])
    )
)]
pub async fn delete_page(
    state: State<AppState>,
    _claims: Claims,
    Path(page_id): Path<Uuid>,
) -> Result<String, BlogError> {
    let pool = &state.pool;

    PageImpl::delete_page(&page_id, pool.clone()).await?;

    Ok(json!({ "ok": "delete page ok!"}).to_string())
}

#[utoipa::path(
    put,
    path = "/page/{page_id}",
    params(
        ("page_id" = String, Path, description = "Page short_id")
    ),
    responses(
        (status = 200, description = "Page update successfully"),
        (status = 500, description = "Internal server error")
    ),
    request_body = UpdatePage,
    security(
        ("jwt_token" = [])
    )
)]
pub async fn update_page(
    state: State<AppState>,
    _claims: Claims,
    Path(page_id): Path<Uuid>,
    Json(update_page): Json<UpdatePage>,
) -> Result<String, BlogError> {
    let pool = &state.pool;

    let page_type = update_page.page;
    match page_type {
        UpdatePageTs::About(about) => {
            let about_page = AboutPage {
                id: Uuid::nil(),
                avatar_url: about.avatar_url,
                content: about.content,
            };
            about_page
                .update_page(
                    &page_id,
                    &update_page.status,
                    &update_page.subname,
                    update_page.comm,
                    pool.clone(),
                )
                .await?;
        }
    }

    Ok(json!({ "ok": "update page ok!"}).to_string())
}
