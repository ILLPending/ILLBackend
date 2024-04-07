use async_trait::async_trait;

use super::{User, UserCreate};

#[async_trait]
pub trait UserRepo {
    async fn create(&self, data: &UserCreate) -> anyhow::Result<User>;

    async fn find_by_discord_id(&self, discord_id: &str) -> anyhow::Result<Option<User>>;
    async fn find_by_id(&self, id: uuid::Uuid) -> anyhow::Result<Option<User>>;

    async fn delete(&self, id: uuid::Uuid) -> anyhow::Result<()>;
}