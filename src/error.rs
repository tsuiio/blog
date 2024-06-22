use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum BlogError {
    #[error("database error: {0}")]
    DatabaseError(#[from] diesel::result::Error),

    #[error("db pooled error: {0}")]
    PooledError(#[from] diesel_async::pooled_connection::PoolError),

    #[error("db pool run error: {0}")]
    PoolRunError(#[from] diesel_async::pooled_connection::bb8::RunError),

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

    #[error("figment error: {0}")]
    FigmentError(#[from] figment::Error),

    #[error("dotenvy error: {0}")]
    DotenvyErrro(#[from] dotenvy::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),

    #[error("wrong credentials")]
    WrongCredentials,

    #[error("missing credentials")]
    MissingCredentials,

    #[error("token creation error")]
    TokenCreation,

    #[error("invalid token")]
    InvalidToken,

    #[error("internal server error")]
    InternalServerError,

    #[error("token expired")]
    TokenExpired,

    #[error("{0}")]
    BadRequest(String),

    #[error("{0}")]
    Unauthorized(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    Conflict(String),
}

impl IntoResponse for BlogError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            BlogError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            BlogError::BadRequest(s) => (StatusCode::BAD_REQUEST, s),
            BlogError::Unauthorized(s) => (StatusCode::UNAUTHORIZED, s),
            BlogError::NotFound(s) => (StatusCode::NOT_FOUND, s),
            BlogError::Conflict(s) => (StatusCode::CONFLICT, s),
            BlogError::WrongCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            BlogError::MissingCredentials => (StatusCode::BAD_REQUEST, self.to_string()),
            BlogError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            BlogError::InvalidToken => (StatusCode::UNAUTHORIZED, self.to_string()),
            BlogError::TokenExpired => (StatusCode::UNAUTHORIZED, self.to_string()),
            _ => {
                error!("{}", self);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("internal server error"),
                )
            }
        };

        let body = json!({ "error": error_message });
        (status, axum::Json(body)).into_response()
    }
}
