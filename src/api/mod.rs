//! HTTP API layer: router, state, handlers.

use axum::{routing::get, Router};
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};

use crate::{config::Config, db::DbPool};

pub mod handlers;
pub mod routes;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub config: Config,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(handlers::health::root))
        .route("/health", get(handlers::health::health))
        .nest("/api/v1", routes::api_v1())
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
}
