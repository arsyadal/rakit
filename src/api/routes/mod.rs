//! Route grouping per resource.

use axum::{
    routing::{delete, get, patch, post, put},
    Router,
};

use crate::api::{handlers, AppState};

pub fn api_v1() -> Router<AppState> {
    // Collection-namespaced content routes
    let collection_routes = Router::new()
        .route("/", post(handlers::content::create).get(handlers::content::list))
        .route(
            "/:id",
            get(handlers::content::get_one)
                .put(handlers::content::update)
                .patch(handlers::content::patch)
                .delete(handlers::content::delete),
        );

    Router::new().nest("/:collection", collection_routes)
}
