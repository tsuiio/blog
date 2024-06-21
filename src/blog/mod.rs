pub mod auth;
pub mod comm;
pub mod info;
pub mod note;
pub mod online;
pub mod page;
pub mod sort;
pub mod tag;
pub mod user;

use auth::login;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use info::{create_info, get_info, update_info};
use note::{create_note, delete_note, get_note, list_notes, update_note};
use page::{create_page, delete_page, get_page, update_page};
use sort::{create_sort, get_sorts, update_sort};
use tag::{create_tag, delete_tag, get_tags, update_tag};
use user::{create_user, update_user_info};
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

use crate::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/user", post(create_user).put(update_user_info))
        .route("/login", post(login))
        .route("/note", post(create_note))
        .route(
            "/note/:id",
            get(get_note).put(update_note).delete(delete_note),
        )
        .route("/notes/:page", get(list_notes))
        .route("/page", post(create_page))
        .route(
            "/page/:id",
            get(get_page).put(update_page).delete(delete_page),
        )
        .route("/tag", post(create_tag))
        .route("/tag/:id", delete(delete_tag).put(update_tag))
        .route("/tags/:page", get(get_tags))
        .route("/sort", post(create_sort))
        .route("/sort/:id", put(update_sort))
        .route("/sorts", get(get_sorts))
        .route("/info", post(create_info).get(get_info).put(update_info))
        .with_state(state)
}

#[derive(OpenApi)]
#[openapi(
        modifiers(&SecurityAddon),
        nest(
            (path = "/api", api = info::InfoDoc),
            (path = "/api", api = user::UserDoc),
            (path = "/api", api = auth::AuthDoc),
            (path = "/api", api = note::NoteDoc),
            (path = "/api", api = tag::TagDoc),
            (path = "/api", api = page::PageDoc),
        ),
        tags(
            (name = "tsuiio's blog", description = "tsuiio's blog API")
        ),
    )]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "jwt_token",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}
