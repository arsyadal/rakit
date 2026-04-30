//! Service functions for the `Content` resource.

use serde_json::Value;
use uuid::Uuid;

use crate::{db::DbPool, errors::ApiError, models::content::Content};

pub async fn create(pool: &DbPool, data: Value) -> Result<Content, ApiError> {
    let row = sqlx::query_as::<_, Content>(
        r#"
        INSERT INTO contents (id, data)
        VALUES ($1, $2)
        RETURNING id, data, created_at, updated_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(data)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn list(pool: &DbPool) -> Result<Vec<Content>, ApiError> {
    let rows = sqlx::query_as::<_, Content>(
        "SELECT id, data, created_at, updated_at FROM contents ORDER BY created_at DESC LIMIT 100",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get(pool: &DbPool, id: Uuid) -> Result<Content, ApiError> {
    let row = sqlx::query_as::<_, Content>(
        "SELECT id, data, created_at, updated_at FROM contents WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or(ApiError::NotFound)?;
    Ok(row)
}

pub async fn delete(pool: &DbPool, id: Uuid) -> Result<(), ApiError> {
    let result = sqlx::query("DELETE FROM contents WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }
    Ok(())
}

pub async fn update(pool: &DbPool, id: Uuid, new_data: Value) -> Result<Content, ApiError> {
    let row = sqlx::query_as::<_, Content>(
        r#"
        UPDATE contents
        SET data = $1
        WHERE id = $2
        RETURNING id, data, created_at, updated_at
        "#,
    )
    .bind(new_data)
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or(ApiError::NotFound)?;
    Ok(row)
}

pub async fn patch(pool: &DbPool, id: Uuid, partial_data: Value) -> Result<Content, ApiError> {
    let row = sqlx::query_as::<_, Content>(
        r#"
        UPDATE contents
        SET data = data || $1
        WHERE id = $2
        RETURNING id, data, created_at, updated_at
        "#,
    )
    .bind(partial_data)
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or(ApiError::NotFound)?;
    Ok(row)
}
