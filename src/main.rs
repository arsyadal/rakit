//! RAKIT - A lightweight, high-performance headless CMS engine.
//!
//! Entry point of the application. Initializes config, database, and HTTP server.

use std::net::SocketAddr;

use anyhow::Context;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod api;
mod config;
mod db;
mod errors;
mod middleware;
mod models;
mod services;
mod utils;

use config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env if present
    dotenvy::dotenv().ok();

    // Init tracing
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "rakit=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load config
    let config = Config::from_env().context("failed to load configuration")?;
    tracing::info!("✅ Configuration loaded");

    // Connect DB
    let pool = db::connect(&config.database_url)
        .await
        .context("failed to connect to database")?;
    tracing::info!("✅ Database connected");

    // Run migrations
    db::migrate(&pool).await.context("failed to run migrations")?;
    tracing::info!("✅ Migrations applied");

    // Build app state
    let state = api::AppState {
        pool,
        config: config.clone(),
    };

    // Build router
    let app = api::router(state);

    // Run server
    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    tracing::info!("🚀 RAKIT listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
