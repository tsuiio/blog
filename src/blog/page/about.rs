use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateAbout {
    pub avatar_url: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ReturnAbout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub avatar_url: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateAbout {
    pub avatar_url: String,
    pub content: String,
}
