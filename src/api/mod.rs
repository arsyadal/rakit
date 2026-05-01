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
    let auth_me = Router::new()
        .route("/api/v1/auth/me", get(handlers::auth::me))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            crate::middleware::auth::require_auth,
        ));

    let media_routes = routes::media_routes().route_layer(middleware::from_fn_with_state(
        state.clone(),
        crate::middleware::auth::require_auth,
    ));

    Router::new()
        .route("/", get(handlers::health::root))
        .route("/health", get(handlers::health::health))
        .nest("/api/v1/auth", routes::auth_routes())
        .nest("/api/v1/_schemas", routes::schema_routes())
        .nest("/api/v1/_webhooks", routes::webhook_routes())
        .nest("/api/v1/_media", media_routes)
        .nest("/api/v1/:collection", routes::collection_routes())
        .nest("/api/v1/admin", routes::admin_routes())
        .merge(auth_me)
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
}
