//! Route grouping per resource.

use axum::{routing::{get, post}, Router};

use crate::api::{handlers, AppState};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(handlers::auth::register))
        .route("/login", post(handlers::auth::login))
}

pub fn collection_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(handlers::content::create).get(handlers::content::list))
        .route(
            "/:id",
            get(handlers::content::get_one)
                .put(handlers::content::update)
                .patch(handlers::content::patch)
                .delete(handlers::content::delete),
        )
}

pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/roles", get(handlers::admin::list_roles))
        .route("/permissions", get(handlers::admin::list_permissions))
        .route("/users/:id/role", post(handlers::admin::assign_user_role))
}
