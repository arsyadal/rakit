//! Routes for the dynamic `contents` resource (jsonb-backed).

use axum::{
    routing::{get, post},
    Router,
};

use crate::api::{handlers::content, AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(content::create).get(content::list))
        .route("/:id", get(content::get_one).delete(content::delete))
}
