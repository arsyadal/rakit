//! Handlers for the dynamic `contents` resource.
//!
//! Demonstrates the RAKIT "Core": accept any JSON shape and persist it
//! into Postgres `jsonb`. This is the foundation for dynamic content types.

use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::Value;
use uuid::Uuid;

use crate::{api::AppState, errors::ApiError, models::content::Content, services::content};

pub async fn create(
    State(state): State<AppState>,
    Path(collection): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<Content>, ApiError> {
    let item = content::create(&state.pool, &collection, payload).await?;
    Ok(Json(item))
}

pub async fn list(
    State(state): State<AppState>,
    Path(collection): Path<String>,
) -> Result<Json<Vec<Content>>, ApiError> {
    let items = content::list(&state.pool, &collection).await?;
    Ok(Json(items))
}

pub async fn get_one(
    State(state): State<AppState>,
    Path((collection, id)): Path<(String, Uuid)>,
) -> Result<Json<Content>, ApiError> {
    let item = content::get(&state.pool, &collection, id).await?;
    Ok(Json(item))
}

pub async fn delete(
    State(state): State<AppState>,
    Path((collection, id)): Path<(String, Uuid)>,
) -> Result<Json<Value>, ApiError> {
    content::delete(&state.pool, &collection, id).await?;
    Ok(Json(serde_json::json!({ "deleted": id })))
}

pub async fn update(
    State(state): State<AppState>,
    Path((collection, id)): Path<(String, Uuid)>,
    Json(payload): Json<Value>,
) -> Result<Json<Content>, ApiError> {
    let item = content::update(&state.pool, &collection, id, payload).await?;
    Ok(Json(item))
}

pub async fn patch(
    State(state): State<AppState>,
    Path((collection, id)): Path<(String, Uuid)>,
    Json(payload): Json<Value>,
) -> Result<Json<Content>, ApiError> {
    let item = content::patch(&state.pool, &collection, id, payload).await?;
    Ok(Json(item))
}
