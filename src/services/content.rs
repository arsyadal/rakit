//! Service functions for the `Content` resource.

use serde_json::Value;
use uuid::Uuid;

use crate::{db::DbPool, errors::ApiError, models::content::Content, utils::validate_collection_name};

pub async fn create(pool: &DbPool, collection: &str, data: Value) -> Result<Content, ApiError> {
    validate_collection_name(collection)?;

    let row = sqlx::query_as::<_, Content>(
        r#"
        INSERT INTO contents (id, collection, data)
        VALUES ($1, $2, $3)
        RETURNING id, collection, data, created_at, updated_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(collection)
    .bind(data)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn list(pool: &DbPool, collection: &str) -> Result<Vec<Content>, ApiError> {
    validate_collection_name(collection)?;

    let rows = sqlx::query_as::<_, Content>(
        "SELECT id, collection, data, created_at, updated_at FROM contents WHERE collection = $1 ORDER BY created_at DESC LIMIT 100",
    )
    .bind(collection)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get(pool: &DbPool, collection: &str, id: Uuid) -> Result<Content, ApiError> {
    validate_collection_name(collection)?;

    let row = sqlx::query_as::<_, Content>(
        "SELECT id, collection, data, created_at, updated_at FROM contents WHERE collection = $1 AND id = $2",
    )
    .bind(collection)
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or(ApiError::NotFound)?;
    Ok(row)
}

pub async fn delete(pool: &DbPool, collection: &str, id: Uuid) -> Result<(), ApiError> {
    validate_collection_name(collection)?;

    let result = sqlx::query("DELETE FROM contents WHERE collection = $1 AND id = $2")
        .bind(collection)
        .bind(id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }
    Ok(())
}

pub async fn update(
    pool: &DbPool,
    collection: &str,
    id: Uuid,
    new_data: Value,
) -> Result<Content, ApiError> {
    validate_collection_name(collection)?;

    let row = sqlx::query_as::<_, Content>(
        r#"
        UPDATE contents
        SET data = $1
        WHERE collection = $2 AND id = $3
        RETURNING id, collection, data, created_at, updated_at
        "#,
    )
    .bind(new_data)
    .bind(collection)
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or(ApiError::NotFound)?;
    Ok(row)
}

pub async fn patch(
    pool: &DbPool,
    collection: &str,
    id: Uuid,
    partial_data: Value,
) -> Result<Content, ApiError> {
    validate_collection_name(collection)?;

    let row = sqlx::query_as::<_, Content>(
        r#"
        UPDATE contents
        SET data = data || $1
        WHERE collection = $2 AND id = $3
        RETURNING id, collection, data, created_at, updated_at
        "#,
    )
    .bind(partial_data)
    .bind(collection)
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or(ApiError::NotFound)?;
    Ok(row)
}
