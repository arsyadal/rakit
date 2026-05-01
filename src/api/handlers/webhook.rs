//! Webhook admin endpoints.

use axum::{
    extract::{Path, State},
    http::HeaderMap,
    Json,
};
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

use crate::{api::AppState, errors::ApiError, models::webhook::CreateWebhookRequest, services::webhook};

#[derive(Debug, Serialize)]
pub struct WebhookListItem {
    pub id: Uuid,
    pub collection: String,
    pub event: String,
    pub url: String,
    pub enabled: bool,
}

pub async fn list(State(state): State<AppState>, headers: HeaderMap) -> Result<Json<Vec<WebhookListItem>>, ApiError> {
    crate::middleware::rbac::authorize_admin(&state, &headers).await?;
    let rows = webhook::list_webhooks(&state.pool).await?;
    Ok(Json(rows.into_iter().map(|w| WebhookListItem {
        id: w.id,
        collection: w.collection,
        event: w.event,
        url: w.url,
        enabled: w.enabled,
    }).collect()))
}

pub async fn create(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateWebhookRequest>,
) -> Result<Json<Value>, ApiError> {
    crate::middleware::rbac::authorize_admin(&state, &headers).await?;
    let row = webhook::create_webhook(&state.pool, &payload.collection, &payload.event, &payload.url, &payload.secret).await?;
    Ok(Json(serde_json::json!({
        "id": row.id,
        "collection": row.collection,
        "event": row.event,
        "url": row.url,
        "enabled": row.enabled,
    })))
}

pub async fn delete(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    crate::middleware::rbac::authorize_admin(&state, &headers).await?;
    webhook::delete_webhook(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "deleted": id })))
}
