use serde::{Deserialize, Serialize};

pub mod repo;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    
    pub discord_id: String,
    pub name: String,
    // This should be a full avatar URL (excluding the size query param)
    pub image: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gd_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gd_name: Option<String>,

    pub created_at: time::OffsetDateTime
}

#[derive(Serialize)]
pub struct UserCreate {
    pub discord_id: String,
    pub name: String,
    pub image: String,
    pub gd_id: Option<String>,
    pub gd_name: Option<String>
}