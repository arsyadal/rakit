//! Route grouping per resource.

use axum::Router;

use crate::api::AppState;

pub mod content;

pub fn api_v1() -> Router<AppState> {
    Router::new().nest("/contents", content::routes())
}
