use async_trait::async_trait;

#[async_trait]
pub trait UserRepo {
    async fn create(&self, email: &str, password: &str) -> anyhow::Result<()>;

    async fn find_by_discord_id(&self, discord_id: &str) -> anyhow::Result<Option<()>>;
    async fn find_by_id(&self, id: uuid::Uuid) -> anyhow::Result<Option<()>>;

    async fn delete(&self, id: uuid::Uuid) -> anyhow::Result<()>;
}