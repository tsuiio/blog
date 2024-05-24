use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlogError {
    #[error("database error: {0}")]
    DatabaseError(#[from] diesel::result::Error),

    #[error("db pooled error: {0}")]
    PooledError(#[from] diesel_async::pooled_connection::PoolError),

    #[error("migrations error: {0}")]
    MigrationError(String),

    #[error("JSON web token error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error("chrono parsing error: {0}")]
    ChronoParseError(#[from] chrono::ParseError),

    #[error("UUID error: {0}")]
    UuidError(#[from] uuid::Error),

    #[error("serialization/deserialization error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("letter error: {0}")]
    LettreError(#[from] lettre::error::Error),

    #[error("diesel connection error: {0}")]
    DiseleError(#[from] diesel::ConnectionError),

    #[error("toml de error: {0}")]
    TomlDeError(#[from] toml::de::Error),

    #[error("setting default subscriber failed: {0}")]
    SubscriberError(#[from] tracing::subscriber::SetGlobalDefaultError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),

    #[error("Internal server error")]
    InternalServerError,

    #[error("{0}")]
    BadRequest(String),

    #[error("{0}")]
    Unauthorized(String),

    #[error("{0}")]
    NotFound(String),
}

impl IntoResponse for BlogError {
    fn into_response(self) -> Response {
        let status = match self {
            BlogError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            BlogError::BadRequest(_) => StatusCode::BAD_REQUEST,
            BlogError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            BlogError::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let error_message = self.to_string();
        let body = json!({ "error": error_message });
        (status, axum::Json(body)).into_response()
    }
}
