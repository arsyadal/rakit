//! Health check & root endpoints.

use axum::Json;
use serde_json::{json, Value};

pub async fn root() -> Json<Value> {
    Json(json!({
        "name": "RAKIT",
        "version": env!("CARGO_PKG_VERSION"),
        "tagline": "A lightweight, high-performance headless CMS engine."
    }))
}

pub async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
