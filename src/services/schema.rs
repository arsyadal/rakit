//! Schema registry + validation helpers.

use serde_json::Value;

use crate::{
    db::DbPool,
    errors::ApiError,
    models::schema::{CollectionSchema, UpsertSchemaResponse},
    utils::validate_collection_name,
};

pub async fn upsert_schema(
    pool: &DbPool,
    collection: &str,
    schema_json: Value,
) -> Result<UpsertSchemaResponse, ApiError> {
    validate_collection_name(collection)?;
    validate_schema_document(&schema_json)?;

    let row = sqlx::query_as::<_, CollectionSchema>(
        r#"
        INSERT INTO collection_schemas (collection, schema_json)
        VALUES ($1, $2)
        ON CONFLICT (collection)
        DO UPDATE SET
            schema_json = EXCLUDED.schema_json,
            version = collection_schemas.version + 1,
            updated_at = NOW()
        RETURNING id, collection, schema_json, version, created_at, updated_at
        "#,
    )
    .bind(collection)
    .bind(schema_json)
    .fetch_one(pool)
    .await?;

    Ok(UpsertSchemaResponse {
        collection: row.collection,
        version: row.version,
    })
}

pub async fn get_schema(pool: &DbPool, collection: &str) -> Result<CollectionSchema, ApiError> {
    validate_collection_name(collection)?;

    let row = sqlx::query_as::<_, CollectionSchema>(
        "SELECT id, collection, schema_json, version, created_at, updated_at FROM collection_schemas WHERE collection = $1",
    )
    .bind(collection)
    .fetch_optional(pool)
    .await?
    .ok_or(ApiError::NotFound)?;

    Ok(row)
}

pub async fn validate_payload_for_collection(
    pool: &DbPool,
    collection: &str,
    payload: &Value,
) -> Result<(), ApiError> {
    validate_collection_name(collection)?;

    let schema = match get_schema(pool, collection).await {
        Ok(schema) => schema,
        Err(ApiError::NotFound) => return Ok(()),
        Err(err) => return Err(err),
    };

    validate_with_schema(&schema.schema_json, payload)
}

pub fn validate_schema_document(schema_json: &Value) -> Result<(), ApiError> {
    jsonschema::JSONSchema::compile(schema_json)
        .map_err(|e| ApiError::BadRequest(format!("invalid JSON Schema: {e}")))?;
    Ok(())
}

pub fn validate_with_schema(schema_json: &Value, payload: &Value) -> Result<(), ApiError> {
    let validator = jsonschema::JSONSchema::compile(schema_json)
        .map_err(|e| ApiError::BadRequest(format!("invalid stored JSON Schema: {e}")))?;

    if let Err(errors) = validator.validate(payload) {
        let msg = errors
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        return Err(ApiError::BadRequest(format!("schema validation failed: {msg}")));
    }

    Ok(())
}

pub fn merge_patch(base: &Value, patch: &Value) -> Result<Value, ApiError> {
    let base_obj = base
        .as_object()
        .ok_or_else(|| ApiError::BadRequest("PATCH requires object payloads".into()))?;
    let patch_obj = patch
        .as_object()
        .ok_or_else(|| ApiError::BadRequest("PATCH requires object payloads".into()))?;

    let mut merged = base_obj.clone();
    for (k, v) in patch_obj {
        merged.insert(k.clone(), v.clone());
    }
    Ok(Value::Object(merged))
}
