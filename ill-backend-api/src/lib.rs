use axum::Router;
use routes::health;

mod routes;
pub mod state;

/// Get the root router for the API.
pub fn router() -> Router {
    Router::new().nest("/api/health", health::router())
}
