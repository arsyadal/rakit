//! Route grouping per resource.

use axum::{routing::{get, patch, post, put}, Router};

use crate::api::{handlers, AppState};

pub fn api_v1() -> Router<AppState> {
    let auth_routes = Router::new()
        .route("/register", post(handlers::auth::register))
        .route("/login", post(handlers::auth::login));

    let collection_routes = Router::new()
        .route("/", post(handlers::content::create).get(handlers::content::list))
        .route(
            "/:id",
            get(handlers::content::get_one)
                .put(handlers::content::update)
                .patch(handlers::content::patch)
                .delete(handlers::content::delete),
        );

    Router::new()
        .nest("/auth", auth_routes)
        .nest("/:collection", collection_routes)
}
