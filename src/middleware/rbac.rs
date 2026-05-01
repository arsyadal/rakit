//! RBAC authorization helpers.

use axum::http::HeaderMap;
use uuid::Uuid;

use crate::{api::AppState, errors::ApiError, services::{auth, rbac}};

pub async fn authorize_collection_action(
    state: &AppState,
    headers: &HeaderMap,
    action: &str,
    collection: &str,
) -> Result<(), ApiError> {
    let role_name = resolve_role_name(state, headers).await?;
    let allowed = rbac::has_permission(&state.pool, &role_name, action, collection).await?;
    if !allowed {
        return Err(ApiError::Forbidden);
    }
    Ok(())
}

pub async fn authorize_admin(state: &AppState, headers: &HeaderMap) -> Result<(), ApiError> {
    let role_name = resolve_role_name(state, headers).await?;
    if role_name != "admin" {
        return Err(ApiError::Forbidden);
    }
    Ok(())
}

async fn resolve_role_name(state: &AppState, headers: &HeaderMap) -> Result<String, ApiError> {
    let Some(auth_header) = headers.get("Authorization").and_then(|h| h.to_str().ok()) else {
        return Ok("public".to_string());
    };

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(ApiError::Unauthorized)?;
    let claims = auth::verify_token(token, &state.config.jwt_secret)?;
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| ApiError::Unauthorized)?;
    Ok(rbac::get_user_role_name(&state.pool, user_id).await?)
}
