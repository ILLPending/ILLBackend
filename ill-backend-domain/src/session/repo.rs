use async_trait::async_trait;

use super::{Session, SessionCreate};


#[async_trait]
pub trait SessionRepo {
    async fn create(&self, session: SessionCreate) -> anyhow::Result<Session>;
    async fn find_by_token(&self, token: &str) -> anyhow::Result<Option<Session>>;
    async fn delete(&self, id: uuid::Uuid) -> anyhow::Result<()>;
}