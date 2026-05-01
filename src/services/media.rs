//! Media storage + metadata CRUD.

use std::path::{Path, PathBuf};

use axum::body::Bytes;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::{
    db::DbPool,
    errors::ApiError,
    models::{media::{MediaAsset, MediaUploadResponse}, user::User},
    services::rbac,
};

const UPLOAD_DIR: &str = "./uploads";

pub async fn store_upload(
    pool: &DbPool,
    owner: &User,
    filename: &str,
    mime_type: &str,
    bytes: Bytes,
) -> Result<MediaAsset, ApiError> {
    tokio::fs::create_dir_all(UPLOAD_DIR).await?;
    let storage_key = Uuid::new_v4().to_string();
    let path = storage_path(&storage_key);

    let mut file = tokio::fs::File::create(&path).await?;
    file.write_all(&bytes).await?;
    file.flush().await?;

    let asset = sqlx::query_as::<_, MediaAsset>(
        r#"
        INSERT INTO media_assets (id, owner_id, filename, mime_type, size_bytes, storage_key, storage_backend)
        VALUES ($1, $2, $3, $4, $5, $6, 'local')
        RETURNING id, owner_id, filename, mime_type, size_bytes, storage_key, storage_backend, created_at, updated_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(owner.id)
    .bind(filename)
    .bind(mime_type)
    .bind(bytes.len() as i64)
    .bind(&storage_key)
    .fetch_one(pool)
    .await?;

    Ok(asset)
}

pub async fn list_for_user(pool: &DbPool, owner: &User) -> Result<Vec<MediaAsset>, ApiError> {
    let rows = sqlx::query_as::<_, MediaAsset>(
        "SELECT id, owner_id, filename, mime_type, size_bytes, storage_key, storage_backend, created_at, updated_at FROM media_assets WHERE owner_id = $1 ORDER BY created_at DESC",
    )
    .bind(owner.id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_asset(pool: &DbPool, id: Uuid) -> Result<MediaAsset, ApiError> {
    let row = sqlx::query_as::<_, MediaAsset>(
        "SELECT id, owner_id, filename, mime_type, size_bytes, storage_key, storage_backend, created_at, updated_at FROM media_assets WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or(ApiError::NotFound)?;
    Ok(row)
}

pub async fn delete_asset(pool: &DbPool, asset: &MediaAsset) -> Result<(), ApiError> {
    let path = storage_path(&asset.storage_key);
    if Path::new(&path).exists() {
        tokio::fs::remove_file(&path).await.ok();
    }

    sqlx::query("DELETE FROM media_assets WHERE id = $1")
        .bind(asset.id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn download_bytes(asset: &MediaAsset) -> Result<Vec<u8>, ApiError> {
    let path = storage_path(&asset.storage_key);
    let bytes = tokio::fs::read(&path).await?;
    Ok(bytes)
}

pub fn upload_response(asset: &MediaAsset) -> MediaUploadResponse {
    MediaUploadResponse {
        id: asset.id,
        filename: asset.filename.clone(),
        mime_type: asset.mime_type.clone(),
        size_bytes: asset.size_bytes,
        download_url: format!("/api/v1/_media/{}/download", asset.id),
    }
}

pub async fn can_delete(pool: &DbPool, user_id: Uuid) -> Result<bool, ApiError> {
    let role = rbac::get_user_role_name(pool, user_id).await?;
    Ok(role == "admin")
}

fn storage_path(storage_key: &str) -> PathBuf {
    Path::new(UPLOAD_DIR).join(storage_key)
}
