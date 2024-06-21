use axum::extract::Path;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::error;
use uuid::Uuid;

use crate::error::BlogError;
use crate::{db::models::sorts::Sort, AppState};

#[derive(Deserialize)]
pub struct CreateSort {
    pub name: String,
    pub content: String,
    pub sort_order: u32,
    pub parent_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct ReturnSort {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Uuid>,
    name: String,
    content: String,
    sort_order: u32,
    parent_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct ListSort {
    sorts: Vec<ReturnSort>,
}

pub async fn create_sort(
    state: State<AppState>,
    Json(c_sort): Json<CreateSort>,
) -> Result<String, BlogError> {
    let mut conn = state.pool.get_owned().await.map_err(|e| {
        error!("{}", e);
        BlogError::InternalServerError
    })?;

    let name = c_sort.name;
    let content = c_sort.content;
    let sort_order = c_sort.sort_order as i32;
    let parent_id = c_sort.parent_id;

    Sort::create_sort(&name, &content, sort_order, parent_id.as_ref(), &mut conn)
        .await
        .map_err(|e| {
            error!("{}", e);
            BlogError::InternalServerError
        })?;

    Ok(json!({"ok": "create sort ok"}).to_string())
}

pub async fn update_sort(
    Path(u_id): Path<String>,
    state: State<AppState>,
    Json(u_sort): Json<CreateSort>,
) -> Result<String, BlogError> {
    let mut conn = state.pool.get_owned().await.map_err(|e| {
        error!("{}", e);
        BlogError::InternalServerError
    })?;

    let id = &Uuid::parse_str(&u_id)?;
    let name = &u_sort.name;
    let content = &u_sort.content;
    let sort_order = u_sort.sort_order as i32;
    let parent_id = u_sort.parent_id.as_ref();
    Sort::update_sort_by_uuid(id, name, content, sort_order, parent_id, &mut conn)
        .await
        .map_err(|e| {
            error!("{}", e);
            BlogError::InternalServerError
        })?;

    Ok(json!({"ok": "update sort ok"}).to_string())
}

pub async fn get_sorts(state: State<AppState>) -> Result<Json<ListSort>, BlogError> {
    let mut conn = state.pool.get_owned().await.map_err(|e| {
        error!("{}", e);
        BlogError::InternalServerError
    })?;

    let sorts = Sort::get_sorts(&mut conn)
        .await
        .map_err(|e| {
            error!("{}", e);
            BlogError::InternalServerError
        })?
        .into_iter()
        .map(|s| ReturnSort {
            id: Some(s.id),
            name: s.name,
            content: s.content,
            sort_order: s.sort_order as u32,
            parent_id: s.parent_id,
        })
        .collect();

    Ok(Json(ListSort { sorts }))
}
