use std::sync::Arc;

use axum::Extension;
use ill_backend_api::state::AppState;
use tracing_subscriber::EnvFilter;

pub async fn run(port: u16) -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let state = AppState::create(&db_url).await;
    let app = ill_backend_api::router()
        .layer(Extension(Arc::new(state)));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
