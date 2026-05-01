//! Service functions for the `Content` resource.

use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;

use crate::{
    db::DbPool,
    errors::ApiError,
    models::content::Content,
    services::{schema, webhook::{self, WebhookEventKind}},
    utils::{validate_collection_name, validate_identifier},
};

#[derive(Debug, Clone)]
pub struct ListOptions {
    pub filters: Vec<FilterExpr>,
    pub sort: Option<SortSpec>,
    pub limit: u32,
    pub offset: u32,
}

impl Default for ListOptions {
    fn default() -> Self {
        Self {
            filters: vec![],
            sort: None,
            limit: 20,
            offset: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilterExpr {
    pub field: FilterField,
    pub op: FilterOp,
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum FilterField {
    Meta(MetaField),
    Data(String),
}

#[derive(Debug, Clone)]
pub enum MetaField {
    Id,
    CreatedAt,
    UpdatedAt,
}

#[derive(Debug, Clone)]
pub enum FilterOp {
    Eq,
    Ne,
    Contains,
    Gt,
    Gte,
    Lt,
    Lte,
}

#[derive(Debug, Clone)]
pub struct SortSpec {
    pub field: SortField,
    pub desc: bool,
}

#[derive(Debug, Clone)]
pub enum SortField {
    Meta(MetaField),
    Data(String),
}

pub async fn create(pool: &DbPool, collection: &str, data: Value) -> Result<Content, ApiError> {
    validate_collection_name(collection)?;
    schema::validate_payload_for_collection(pool, collection, &data).await?;

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

    emit_webhook_event(pool, collection, WebhookEventKind::Created, row.data.clone());
    Ok(row)
}

pub async fn list(pool: &DbPool, collection: &str, options: ListOptions) -> Result<Vec<Content>, ApiError> {
    validate_collection_name(collection)?;

    let mut qb = QueryBuilder::<Postgres>::new(
        "SELECT id, collection, data, created_at, updated_at FROM contents WHERE collection = ",
    );
    qb.push_bind(collection);

    for filter in options.filters {
        qb.push(" AND ");
        push_filter(&mut qb, filter)?;
    }

    qb.push(" ORDER BY ");
    push_sort(&mut qb, options.sort)?;

    let limit = options.limit.clamp(1, 100);
    qb.push(" LIMIT ");
    qb.push_bind(limit as i64);

    qb.push(" OFFSET ");
    qb.push_bind(options.offset as i64);

    let rows = qb.build_query_as::<Content>().fetch_all(pool).await?;
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

    let current = get(pool, collection, id).await?;
    let result = sqlx::query("DELETE FROM contents WHERE collection = $1 AND id = $2")
        .bind(collection)
        .bind(id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }

    emit_webhook_event(pool, collection, WebhookEventKind::Deleted, current.data.clone());
    Ok(())
}

pub async fn update(
    pool: &DbPool,
    collection: &str,
    id: Uuid,
    new_data: Value,
) -> Result<Content, ApiError> {
    validate_collection_name(collection)?;
    schema::validate_payload_for_collection(pool, collection, &new_data).await?;

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

    emit_webhook_event(pool, collection, WebhookEventKind::Updated, row.data.clone());
    Ok(row)
}

pub async fn patch(
    pool: &DbPool,
    collection: &str,
    id: Uuid,
    partial_data: Value,
) -> Result<Content, ApiError> {
    validate_collection_name(collection)?;

    let current = get(pool, collection, id).await?;
    let merged = schema::merge_patch(&current.data, &partial_data)?;
    schema::validate_payload_for_collection(pool, collection, &merged).await?;

    let row = sqlx::query_as::<_, Content>(
        r#"
        UPDATE contents
        SET data = $1
        WHERE collection = $2 AND id = $3
        RETURNING id, collection, data, created_at, updated_at
        "#,
    )
    .bind(merged.clone())
    .bind(collection)
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or(ApiError::NotFound)?;

    emit_webhook_event(pool, collection, WebhookEventKind::Updated, row.data.clone());
    Ok(row)
}

fn push_filter(qb: &mut QueryBuilder<'_, Postgres>, filter: FilterExpr) -> Result<(), ApiError> {
    match filter.field {
        FilterField::Data(key) => push_data_filter(qb, &key, filter.op, &filter.value),
        FilterField::Meta(meta) => push_meta_filter(qb, meta, filter.op, &filter.value),
    }
}

fn push_data_filter(
    qb: &mut QueryBuilder<'_, Postgres>,
    key: &str,
    op: FilterOp,
    value: &str,
) -> Result<(), ApiError> {
    validate_identifier(key)?;
    let expr = format!("data->>'{}'", key);

    match op {
        FilterOp::Eq => {
            qb.push(&expr);
            qb.push(" = ");
            qb.push_bind(value.to_string());
        }
        FilterOp::Ne => {
            qb.push(&expr);
            qb.push(" <> ");
            qb.push_bind(value.to_string());
        }
        FilterOp::Contains => {
            qb.push(&expr);
            qb.push(" ILIKE ");
            qb.push_bind(format!("%{}%", value));
        }
        FilterOp::Gt | FilterOp::Gte | FilterOp::Lt | FilterOp::Lte => {
            return Err(ApiError::BadRequest(
                "comparison operators gt/gte/lt/lte are only supported for metadata fields in v1".into(),
            ));
        }
    }

    Ok(())
}

fn push_meta_filter(
    qb: &mut QueryBuilder<'_, Postgres>,
    meta: MetaField,
    op: FilterOp,
    value: &str,
) -> Result<(), ApiError> {
    let column = match meta {
        MetaField::Id => "id",
        MetaField::CreatedAt => "created_at",
        MetaField::UpdatedAt => "updated_at",
    };

    match meta {
        MetaField::Id => {
            let parsed = Uuid::parse_str(value)
                .map_err(|_| ApiError::BadRequest("invalid UUID for id filter".into()))?;
            match op {
                FilterOp::Eq => {
                    qb.push(column);
                    qb.push(" = ");
                    qb.push_bind(parsed);
                }
                FilterOp::Ne => {
                    qb.push(column);
                    qb.push(" <> ");
                    qb.push_bind(parsed);
                }
                _ => {
                    return Err(ApiError::BadRequest(
                        "only eq/ne are supported for id filters".into(),
                    ));
                }
            }
        }
        MetaField::CreatedAt | MetaField::UpdatedAt => {
            let parsed = DateTime::parse_from_rfc3339(value)
                .map_err(|_| ApiError::BadRequest("invalid RFC3339 datetime".into()))?
                .with_timezone(&Utc);

            qb.push(column);
            match op {
                FilterOp::Eq => {
                    qb.push(" = ");
                }
                FilterOp::Ne => {
                    qb.push(" <> ");
                }
                FilterOp::Gt => {
                    qb.push(" > ");
                }
                FilterOp::Gte => {
                    qb.push(" >= ");
                }
                FilterOp::Lt => {
                    qb.push(" < ");
                }
                FilterOp::Lte => {
                    qb.push(" <= ");
                }
                FilterOp::Contains => {
                    return Err(ApiError::BadRequest(
                        "contains is not supported for datetime filters".into(),
                    ));
                }
            }
            qb.push_bind(parsed);
        }
    }

    Ok(())
}

fn push_sort(qb: &mut QueryBuilder<'_, Postgres>, sort: Option<SortSpec>) -> Result<(), ApiError> {
    let spec = sort.unwrap_or(SortSpec {
        field: SortField::Meta(MetaField::CreatedAt),
        desc: true,
    });

    match spec.field {
        SortField::Meta(meta) => {
            let column = match meta {
                MetaField::Id => "id",
                MetaField::CreatedAt => "created_at",
                MetaField::UpdatedAt => "updated_at",
            };
            qb.push(column);
        }
        SortField::Data(key) => {
            validate_identifier(&key)?;
            qb.push(format!("data->>'{}'", key));
        }
    }

    if spec.desc {
        qb.push(" DESC");
    } else {
        qb.push(" ASC");
    }

    Ok(())
}

fn emit_webhook_event(pool: &DbPool, collection: &str, event: WebhookEventKind, content: Value) {
    let pool = pool.clone();
    let collection = collection.to_string();
    tokio::spawn(async move {
        webhook::emit_event(pool, collection, event, content).await;
    });
}
