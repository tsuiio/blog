use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    Json,
};
use chrono::NaiveDateTime;
use futures_util::future::try_join;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::error;
use utoipa::{OpenApi, ToSchema};
use uuid::Uuid;

use crate::{
    db::models::{
        note_sorts::NoteSort,
        note_tags::NoteTag,
        notes::{Note, Status},
    },
    error::BlogError,
    utils::{extract_summary, jwt::Claims},
    AppState,
};

#[derive(Deserialize, ToSchema)]
pub struct CreateNote {
    pub title: String,
    pub subname: Option<String>,
    pub status: Status,
    pub content: String,
    pub comm: bool,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateNote {
    pub title: String,
    pub subname: Option<String>,
    pub status: Status,
    pub content: String,
    pub comm: bool,
}

#[derive(Serialize, ToSchema)]
pub struct ReturnNote {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<Uuid>,
    title: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<Status>,
    summary: String,
    content: String,
    comm: bool,
    tags: Vec<String>,
    sorts: Vec<ListNoteSorts>,
}

#[derive(Serialize, ToSchema)]
pub struct ListNoteInner {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub title: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    pub short_id: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub sorts: Vec<ListNoteSorts>,
}

#[derive(Serialize, ToSchema)]
pub struct ListNoteSorts {
    pub subname: String,
    pub content: String,
}

#[derive(Serialize, ToSchema)]
pub struct ListNotes {
    pub notes: Vec<ListNoteInner>,
    pub total: u64,
}

#[derive(OpenApi)]
#[openapi(
    paths(create_note, update_note, get_note, list_notes, delete_note),
    components(schemas(
        CreateNote,
        UpdateNote,
        ListNoteInner,
        ListNoteSorts,
        ListNotes,
        ReturnNote,
        Status
    ))
)]
pub struct NoteDoc;

#[utoipa::path(
    post,
    path = "/note",
    request_body = CreateNote,
    responses(
        (status = 200, description = "Note created successfully"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt_token" = [])
    )
)]
pub async fn create_note(
    state: State<AppState>,
    claims: Claims,
    Json(new_note): Json<CreateNote>,
) -> Result<String, BlogError> {
    let pool = &state.pool;

    let user_id = Uuid::parse_str(&claims.user_id)?;
    let summary = extract_summary(&new_note.content, 40);

    if let Some(subname) = &new_note.subname {
        let mut conn = pool.get_owned().await?;
        let have_note = Note::find_note_by_short_id(subname, &mut conn)
            .await
            .map_err(|e| {
                error!("{}", e);
                BlogError::InternalServerError
            })?
            .is_some();
        if have_note {
            return Err(BlogError::Conflict("subname already exists".to_string()));
        }
    }

    Note::create_note(
        new_note.subname.as_deref(),
        &new_note.status,
        &new_note.title,
        &summary,
        &new_note.content,
        new_note.comm,
        &user_id,
        pool.clone(),
    )
    .await
    .map_err(|e| {
        error!("{}", e);
        BlogError::InternalServerError
    })?;

    Ok(json!({ "ok": "create note ok!"}).to_string())
}

#[utoipa::path(
    put,
    path = "/note/{note_id}",
    params(
        ("note_id" = Uuid, Path, description = "Tag id")
    ),
    responses(
        (status = 200, description = "Note update successfully"),
        (status = 500, description = "Internal server error")
    ),
    request_body = UpdateNote,
    security(
        ("jwt_token" = [])
    )
)]
pub async fn update_note(
    state: State<AppState>,
    _claims: Claims,
    Path(note_id): Path<Uuid>,
    Json(u_note): Json<UpdateNote>,
) -> Result<String, BlogError> {
    let pool = &state.pool;

    let summary = extract_summary(&u_note.content, 40);

    Note::update_note_by_uuid(
        &note_id,
        &u_note.title,
        u_note.subname.as_deref(),
        &summary,
        &u_note.content,
        &u_note.status,
        pool.clone(),
    )
    .await
    .map_err(|e| {
        error!("{}", e);
        BlogError::InternalServerError
    })?;

    Ok(json!({ "ok": "update note ok!"}).to_string())
}

#[utoipa::path(
    get,
    path = "/notes/{page}",
    params(
        ("page" = u64, Path, description = "List notes by page")
    ),
    responses(
        (status = 200, description = "Info retrieved successfully", body = ListNotes),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal server error"),
    ),
    security(
        (),
        ("jwt_token" = [])
    )
)]
pub async fn list_notes(
    state: State<AppState>,
    claims: Option<Claims>,
    Path(page): Path<u64>,
) -> Result<Json<ListNotes>, BlogError> {
    if page == 0 {
        return Err(BlogError::BadRequest(String::from("not 0!")));
    }

    let mut conn = state.pool.get_owned().await.map_err(|e| {
        error!("{}", e);
        BlogError::InternalServerError
    })?;

    let limit = 10;
    let offset = (page - 1) * limit;
    let (notes, short_names, total) =
        Note::get_notes_short_total(limit as i64, offset as i64, &mut conn)
            .await
            .map_err(|e| {
                error!("{}", e);
                BlogError::InternalServerError
            })?;

    let note_tags = NoteTag::get_note_tags_and_tags_by_notes(&notes, &mut conn);

    let mut conn = state.pool.get_owned().await.map_err(|e| {
        error!("{}", e);
        BlogError::InternalServerError
    })?;
    let note_sorts = NoteSort::get_note_sorts_and_tags_by_notes(&notes, &mut conn);

    let (note_tags, note_sorts) = try_join(note_tags, note_sorts).await.map_err(|e| {
        error!("{}", e);
        BlogError::InternalServerError
    })?;

    let mut tags_map: HashMap<Uuid, Vec<String>> = HashMap::new();
    for (note_tag, tag) in note_tags {
        tags_map
            .entry(note_tag.note_id)
            .or_default()
            .push(tag.content);
    }

    let mut sorts_map: HashMap<Uuid, Vec<ListNoteSorts>> = HashMap::new();
    for (note_sort, sort) in note_sorts {
        sorts_map
            .entry(note_sort.note_id)
            .or_default()
            .push(ListNoteSorts {
                subname: sort.name,
                content: sort.content,
            });
    }

    let is_authenticated = claims.is_some();

    let list_notes: Vec<ListNoteInner> = notes
        .into_iter()
        .zip(short_names)
        .map(|(note, short_name)| ListNoteInner {
            id: if is_authenticated {
                Some(note.id)
            } else {
                None
            },
            title: note.title,
            created_at: note.created_at,
            updated_at: note.updated_at,
            status: if is_authenticated {
                Some(note.status)
            } else {
                None
            },
            short_id: short_name.1.unwrap_or(short_name.0),
            summary: note.summary,
            tags: tags_map.remove(&note.id).unwrap_or_default(),
            sorts: sorts_map.remove(&note.id).unwrap_or_default(),
        })
        .collect();

    Ok(Json(ListNotes {
        notes: list_notes,
        total,
    }))
}

#[utoipa::path(
    delete,
    path = "/note/{note_id}",
    params(
        ("note_id" = Uuid, Path, description = "Note id")
    ),
    responses(
        (status = 200, description = "Note delete successfully"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("jwt_token" = [])
    )
)]
pub async fn delete_note(
    state: State<AppState>,
    _claims: Claims,
    Path(note_id): Path<Uuid>,
) -> Result<String, BlogError> {
    let pool = &state.pool;

    // let note_id = Uuid::parse_str(&note_id)?;
    Note::delete_note_by_uuid(&note_id, pool.clone())
        .await
        .map_err(|e| {
            error!("delete note {} error: {}", note_id, e);
            BlogError::InternalServerError
        })?;

    Ok(json!({ "ok": "delete note ok!"}).to_string())
}

#[utoipa::path(
    get,
    path = "/note/{short_id}",
    params(
        ("short_id" = String, Path, description = "Note short_id")
    ),
    responses(
        (status = 200, description = "Note get successfully", body = ReturnNote),
        (status = 404, description = "Note not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        (),
        ("jwt_token" = [])
    )
)]
pub async fn get_note(
    state: State<AppState>,
    claims: Option<Claims>,
    Path(short_id): Path<String>,
) -> Result<Json<ReturnNote>, BlogError> {
    let mut conn = state.pool.get_owned().await.map_err(|e| {
        error!("{}", e);
        BlogError::InternalServerError
    })?;

    let note = Note::find_note_by_short_id(&short_id, &mut conn)
        .await
        .map_err(|e| {
            error!("{}", e);
            BlogError::InternalServerError
        })?
        .ok_or(BlogError::NotFound(String::from("not found note")))?;

    let tags = NoteTag::get_tags_by_note_id(&note.id, &mut conn);

    let mut conn = state.pool.get_owned().await.map_err(|e| {
        error!("{}", e);
        BlogError::InternalServerError
    })?;
    let sorts = NoteSort::get_sorts_by_note_id(&note.id, &mut conn);

    let (tags, sorts) = try_join(tags, sorts).await.map_err(|e| {
        error!("{}", e);
        BlogError::InternalServerError
    })?;

    let is_authenticated = claims.is_some();

    Ok(Json(ReturnNote {
        id: if is_authenticated {
            Some(note.id)
        } else {
            None
        },
        title: note.title,
        created_at: note.created_at,
        updated_at: note.updated_at,
        status: if is_authenticated {
            Some(note.status)
        } else {
            None
        },
        summary: note.summary,
        content: note.content,
        comm: note.comm,
        tags: tags.into_iter().map(|t| t.content).collect(),
        sorts: sorts
            .into_iter()
            .map(|s| ListNoteSorts {
                subname: s.name,
                content: s.content,
            })
            .collect(),
    }))
}
