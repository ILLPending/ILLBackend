use async_trait::async_trait;
use ill_backend_domain::user::{repo::UserRepo, User, UserCreate};


pub struct ILLUserRepo {
    db: sqlx::PgPool
}

impl ILLUserRepo {
    #[must_use]
    pub fn new(db: sqlx::PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepo for ILLUserRepo {
    async fn create(&self, data: &UserCreate) -> anyhow::Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            insert into users (discord_id, name, image, gd_id, gd_name)
            values ($1, $2, $3, $4, $5)
            returning id, discord_id, name, image, gd_id, gd_name, created_at
            "#,
            data.discord_id,
            data.name,
            data.image,
            data.gd_id,
            data.gd_name
        )
        .fetch_one(&self.db)
        .await?;

        Ok(user)
    }

    async fn find_by_discord_id(&self, discord_id: &str) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            select id, discord_id, name, image, gd_id, gd_name, created_at
            from users
            where discord_id = $1
            "#,
            discord_id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(user)
    }

    async fn find_by_id(&self, id: uuid::Uuid) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            select id, discord_id, name, image, gd_id, gd_name, created_at
            from users
            where id = $1
            "#,
            id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(user)
    }
    
    async fn delete(&self, id: uuid::Uuid) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            delete from users
            where id = $1
            "#,
            id
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }
}