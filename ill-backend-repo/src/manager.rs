use crate::repos;


pub struct RepositoryManager {
    pub user_repo: repos::user::ILLUserRepo,
    pub session_repo: repos::session::ILLSessionRepo
}

impl RepositoryManager {
    #[must_use]
    ///
    /// # Panics
    /// Panics if it fails to connect to the database
    /// 
    pub async fn create(database_url: &str) -> Self {
        let db = sqlx::PgPool::connect(database_url).await.expect("Failed to connect to database");

        Self {
            user_repo: repos::user::ILLUserRepo::new(db.clone()),
            session_repo: repos::session::ILLSessionRepo::new(db.clone())
        }
    }
}