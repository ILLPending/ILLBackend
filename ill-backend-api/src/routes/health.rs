use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new().route("/", get(health_check))
}

async fn health_check() -> &'static str {
    "Alive"
}
