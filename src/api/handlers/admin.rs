//! Admin RBAC handlers.

use axum::{extract::{Path, State}, http::HeaderMap, Json};
use serde_json::Value;
use uuid::Uuid;

use crate::{
    api::AppState,
    errors::ApiError,
    models::rbac::{AssignRoleRequest, Permission, Role},
    services::{rbac, rbac as rbac_service},
};

pub async fn list_roles(State(state): State<AppState>, headers: HeaderMap) -> Result<Json<Vec<Role>>, ApiError> {
    crate::middleware::rbac::authorize_admin(&state, &headers).await?;
    let roles = rbac::list_roles(&state.pool).await?;
    Ok(Json(roles))
}

pub async fn list_permissions(State(state): State<AppState>, headers: HeaderMap) -> Result<Json<Vec<Permission>>, ApiError> {
    crate::middleware::rbac::authorize_admin(&state, &headers).await?;
    let permissions = rbac::list_permissions(&state.pool).await?;
    Ok(Json(permissions))
}

pub async fn assign_user_role(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<AssignRoleRequest>,
) -> Result<Json<Value>, ApiError> {
    crate::middleware::rbac::authorize_admin(&state, &headers).await?;
    rbac_service::assign_role_to_user(&state.pool, user_id, &payload.role).await?;
    Ok(Json(serde_json::json!({ "user_id": user_id, "role": payload.role })))
}
