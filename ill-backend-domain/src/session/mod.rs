use serde::{Deserialize, Serialize};

pub mod repo;

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub token: String,
    pub expires_at: time::OffsetDateTime,
    pub created_at: time::OffsetDateTime
}

#[derive(Serialize)]
pub struct SessionCreate {
    pub user_id: uuid::Uuid,
    pub token: String,
    pub expires_at: time::OffsetDateTime
}