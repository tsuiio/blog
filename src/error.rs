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

    #[error("Internal server error")]
    InternalServerError,

    #[error("Bad request")]
    BadRequest,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Not found")]
    NotFound,
}

impl IntoResponse for BlogError {
    fn into_response(self) -> Response {
        let status = match self {
            BlogError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            BlogError::BadRequest => StatusCode::BAD_REQUEST,
            BlogError::Unauthorized => StatusCode::UNAUTHORIZED,
            BlogError::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let error_message = self.to_string();
        let body = json!({ "error": error_message });
        (status, axum::Json(body)).into_response()
    }
}
