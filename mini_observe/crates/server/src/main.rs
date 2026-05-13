use anyhow::Result;
use miniobserve_api::{build_router, AppState};
use miniobserve_common::AppConfig;
use miniobserve_ingest::start_ingestion_pipeline;
use miniobserve_query::QueryEngine;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let config = AppConfig::default();

    let ingest = start_ingestion_pipeline(config.clone()).await?;
    let query = QueryEngine::new(config.parquet_dir.clone());

    let state = AppState { ingest, query };

    let app = build_router(state);

    let addr: SocketAddr = "0.0.0.0:3000".parse()?;
    let listener = TcpListener::bind(addr).await?;

    info!("miniobserve server running on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}