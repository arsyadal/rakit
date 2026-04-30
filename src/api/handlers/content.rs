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
    Json(payload): Json<Value>,
) -> Result<Json<Content>, ApiError> {
    let item = content::create(&state.pool, payload).await?;
    Ok(Json(item))
}

pub async fn list(State(state): State<AppState>) -> Result<Json<Vec<Content>>, ApiError> {
    let items = content::list(&state.pool).await?;
    Ok(Json(items))
}

pub async fn get_one(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Content>, ApiError> {
    let item = content::get(&state.pool, id).await?;
    Ok(Json(item))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    content::delete(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "deleted": id })))
}
