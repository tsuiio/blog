use axum::{
    extract::{Path, State},
    Json,
};
use chrono::{NaiveDateTime};
use serde::{Deserialize};
use uuid::Uuid;

use crate::{error::BlogError, AppState};

#[derive(Deserialize)]
pub struct CreateComm {
    pub content: String,
    pub comm_user_email: String,
    pub note_id: Option<Uuid>,
    pub page_id: Option<Uuid>,
}

pub struct ReturnComm {
    id: Uuid,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    nickname: String,
    content: String,
    website_url: Option<String>,
    note_id: Option<Uuid>,
    page_id: Option<Uuid>,
}

pub struct UpdateComm {
    content: String,
    nickname: String,
    website_url: Option<String>,
    note_id: Option<Uuid>,
    page_id: Option<Uuid>,
}

pub struct ListComms {
    pub comms: Vec<ReturnComm>,
    pub total: u64,
}

pub async fn create_comm(
    state: State<AppState>,
    Json(new_comm): Json<CreateComm>,
) -> Result<String, BlogError> {
    todo!()
}

pub async fn update_comm(
    state: State<AppState>,
    Path(comm_id): Path<String>,
) -> Result<String, BlogError> {
    todo!()
}

pub async fn get_comm(
    state: State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ListComms>, BlogError> {
    todo!()
}

pub async fn delete_comm(
    state: State<AppState>,
    Path(comm_id): Path<String>,
) -> Result<String, BlogError> {
    todo!()
}
