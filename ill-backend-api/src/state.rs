
pub struct AppState {
    pub manager: ill_backend_repo::manager::RepositoryManager
}

impl AppState {
    #[must_use]
    pub async fn create(database_url: &str) -> Self {
        let manager = ill_backend_repo::manager::RepositoryManager::create(database_url).await;

        Self { manager }
    }
}