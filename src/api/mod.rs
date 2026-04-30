//! HTTP API layer: router, state, handlers.

use axum::{middleware, routing::get, Router};
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
    let protected_routes = Router::new()
        .route("/api/v1/auth/me", get(handlers::auth::me))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            crate::middleware::auth::require_auth,
        ));

    Router::new()
        .route("/", get(handlers::health::root))
        .route("/health", get(handlers::health::health))
        .nest("/api/v1", routes::api_v1())
        .merge(protected_routes)
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
}
