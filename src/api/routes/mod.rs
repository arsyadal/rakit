//! Route grouping per resource.

use axum::{routing::post, Router};

use crate::api::{handlers, AppState};

pub mod content;

pub fn api_v1() -> Router<AppState> {
    let auth_routes = Router::new()
        .route("/register", post(handlers::auth::register))
        .route("/login", post(handlers::auth::login));

    Router::new()
        .nest("/auth", auth_routes)
        .nest("/contents", content::routes())
}
