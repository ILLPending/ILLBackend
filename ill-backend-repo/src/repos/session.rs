use async_trait::async_trait;
use ill_backend_domain::session::{repo::SessionRepo, Session, SessionCreate};



pub struct ILLSessionRepo {
    db: sqlx::PgPool
}

impl ILLSessionRepo {
    #[must_use]
    pub fn new(db: sqlx::PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl SessionRepo for ILLSessionRepo {
    async fn create(&self, session: SessionCreate) -> anyhow::Result<Session> {
        let session = sqlx::query_as!(
            Session,
            r#"
            insert into sessions (user_id, token, expires_at)
            values ($1, $2, $3)
            returning id, user_id, token, expires_at, created_at
            "#,
            session.user_id,
            session.token,
            session.expires_at
        )
        .fetch_one(&self.db)
        .await?;
        Ok(session)
    }

    async fn find_by_token(&self, token: &str) -> anyhow::Result<Option<Session>> {
        let session = sqlx::query_as!(
            Session,
            r#"
            select id, user_id, token, expires_at, created_at
            from sessions
            where token = $1
            "#,
            token
        )
        .fetch_optional(&self.db)
        .await?;
        Ok(session)
    }

    async fn delete(&self, id: uuid::Uuid) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            delete from sessions
            where id = $1
            "#,
            id
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }
}