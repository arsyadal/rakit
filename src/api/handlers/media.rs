//! Media upload endpoints.

use axum::{
    body::{Body, Bytes},
    extract::{Multipart, Path, State},
    http::header,
    response::Response,
    Json,
};
use serde_json::Value;
use uuid::Uuid;

use crate::{
    api::AppState,
    errors::ApiError,
    models::{media::MediaUploadResponse, user::User},
    services::media,
};

const MAX_UPLOAD_BYTES: usize = 10 * 1024 * 1024; // 10MB

pub async fn upload(
    State(state): State<AppState>,
    axum::Extension(user): axum::Extension<User>,
    mut multipart: Multipart,
) -> Result<Json<MediaUploadResponse>, ApiError> {
    let mut filename: Option<String> = None;
    let mut mime_type: Option<String> = None;
    let mut data: Option<Bytes> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| ApiError::BadRequest(e.to_string()))? {
        if field.name() != Some("file") {
            continue;
        }
        filename = field.file_name().map(|s| s.to_string());
        mime_type = field.content_type().map(|s| s.to_string());
        let bytes = field.bytes().await.map_err(|e| ApiError::BadRequest(e.to_string()))?;
        if bytes.len() > MAX_UPLOAD_BYTES {
            return Err(ApiError::BadRequest("file too large (max 10MB)".into()));
        }
        data = Some(bytes);
        break;
    }

    let data = data.ok_or_else(|| ApiError::BadRequest("missing file field".into()))?;
    let filename = filename.unwrap_or_else(|| "upload.bin".to_string());
    let mime_type = mime_type.unwrap_or_else(|| "application/octet-stream".to_string());

    let asset = media::store_upload(&state.pool, &user, &filename, &mime_type, data).await?;
    Ok(Json(media::upload_response(&asset)))
}

pub async fn list(
    State(state): State<AppState>,
    axum::Extension(user): axum::Extension<User>,
) -> Result<Json<Vec<MediaUploadResponse>>, ApiError> {
    let rows = media::list_for_user(&state.pool, &user).await?;
    Ok(Json(rows.iter().map(media::upload_response).collect()))
}

pub async fn get(
    State(state): State<AppState>,
    axum::Extension(user): axum::Extension<User>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let asset = media::get_asset(&state.pool, id).await?;
    if asset.owner_id != user.id && !media::can_delete(&state.pool, user.id).await? {
        return Err(ApiError::Forbidden);
    }

    Ok(Json(serde_json::json!({
        "id": asset.id,
        "owner_id": asset.owner_id,
        "filename": asset.filename,
        "mime_type": asset.mime_type,
        "size_bytes": asset.size_bytes,
        "storage_backend": asset.storage_backend,
        "created_at": asset.created_at,
        "updated_at": asset.updated_at,
        "download_url": format!("/api/v1/_media/{}/download", asset.id),
    })))
}

pub async fn download(
    State(state): State<AppState>,
    axum::Extension(user): axum::Extension<User>,
    Path(id): Path<Uuid>,
) -> Result<Response, ApiError> {
    let asset = media::get_asset(&state.pool, id).await?;
    if asset.owner_id != user.id && !media::can_delete(&state.pool, user.id).await? {
        return Err(ApiError::Forbidden);
    }

    let bytes = media::download_bytes(&asset).await?;
    let mut resp = Response::new(Body::from(bytes));
    let headers = resp.headers_mut();
    headers.insert(header::CONTENT_TYPE, asset.mime_type.parse().unwrap_or(header::HeaderValue::from_static("application/octet-stream")));
    headers.insert(header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", asset.filename).parse().unwrap());
    Ok(resp)
}

pub async fn delete(
    State(state): State<AppState>,
    axum::Extension(user): axum::Extension<User>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let asset = media::get_asset(&state.pool, id).await?;
    if asset.owner_id != user.id && !media::can_delete(&state.pool, user.id).await? {
        return Err(ApiError::Forbidden);
    }

    media::delete_asset(&state.pool, &asset).await?;
    Ok(Json(serde_json::json!({ "deleted": id })))
}
