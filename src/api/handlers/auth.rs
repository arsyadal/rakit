//! Auth HTTP handlers.

use axum::{extract::State, http::StatusCode, Extension, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    api::AppState,
    errors::ApiError,
    models::user::User,
    services::auth,
};

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

/// POST /auth/register
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<User>, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let user = auth::register(&state.pool, &payload.email, &payload.password).await?;
    Ok(Json(user))
}

/// POST /auth/login
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    payload
        .validate()
        .map_err(|e| ApiError::BadRequest(e.to_string()))?;

    let (token, user) = auth::login(
        &state.pool,
        &payload.email,
        &payload.password,
        &state.config.jwt_secret,
        state.config.jwt_expiration_hours,
    )
    .await?;

    Ok(Json(LoginResponse { token, user }))
}

/// GET /auth/me — requires JWT middleware
pub async fn me(Extension(user): Extension<User>) -> Result<(StatusCode, Json<User>), ApiError> {
    // User already fetched by middleware
    Ok((StatusCode::OK, Json(user)))
}
