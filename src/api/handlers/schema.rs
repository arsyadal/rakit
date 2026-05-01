//! Schema registry endpoints.

use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};
use serde_json::Value;

use crate::{api::AppState, errors::ApiError, services::schema};

pub async fn upsert(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(collection): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, ApiError> {
    crate::middleware::rbac::authorize_admin(&state, &headers).await?;
    let response = schema::upsert_schema(&state.pool, &collection, payload).await?;
    Ok(Json(serde_json::json!({
        "collection": response.collection,
        "version": response.version,
    })))
}

pub async fn get(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(collection): Path<String>,
) -> Result<Json<Value>, ApiError> {
    crate::middleware::rbac::authorize_admin(&state, &headers).await?;
    let row = schema::get_schema(&state.pool, &collection).await?;
    Ok(Json(serde_json::json!({
        "collection": row.collection,
        "schema_json": row.schema_json,
        "version": row.version,
    })))
}
