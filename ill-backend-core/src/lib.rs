use axum::Router;

pub async fn run(port: u16) -> anyhow::Result<()> {
    let app = Router::new();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
