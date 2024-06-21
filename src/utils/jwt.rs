use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::Utc;
use jsonwebtoken::Algorithm;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

use crate::{config::CONFIG, error::BlogError};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    exp: usize,
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User ID: {}", self.user_id)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = BlogError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| BlogError::InvalidToken)?;

        let token = decode_jwt(bearer.token())?;

        Ok(token)
    }
}

pub fn encode_jwt(id: &Uuid) -> Result<String, BlogError> {
    let secret = CONFIG.web.jwt_secret.as_ref().unwrap();

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(CONFIG.web.jwt_exp.unwrap() as i64))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        user_id: id.to_string(),
        exp: expiration as usize,
    };

    let header = Header::new(jsonwebtoken::Algorithm::HS512);

    Ok(encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?)
}

pub fn decode_jwt(token: &str) -> Result<Claims, BlogError> {
    let secret = CONFIG.web.jwt_secret.as_ref().unwrap();

    let token = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|_| BlogError::InvalidToken)?;

    Ok(token.claims)
}
