//! Webhook registry and delivery.

use std::time::Duration;

use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::Sha256;
use uuid::Uuid;

use crate::{
    db::DbPool,
    errors::ApiError,
    models::webhook::Webhook,
    utils::validate_collection_name,
};

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebhookEventKind {
    Created,
    Updated,
    Deleted,
}

impl WebhookEventKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Updated => "updated",
            Self::Deleted => "deleted",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEventPayload {
    pub event: String,
    pub collection: String,
    pub content: Value,
    pub timestamp: String,
}

pub async fn list_webhooks(pool: &DbPool) -> Result<Vec<Webhook>, ApiError> {
    let rows = sqlx::query_as::<_, Webhook>(
        "SELECT id, collection, event, url, secret, enabled, created_at, updated_at FROM webhooks ORDER BY created_at DESC",
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn create_webhook(
    pool: &DbPool,
    collection: &str,
    event: &str,
    url: &str,
    secret: &str,
) -> Result<Webhook, ApiError> {
    validate_webhook_collection(collection)?;
    validate_webhook_event(event)?;
    validate_webhook_url(url)?;

    let row = sqlx::query_as::<_, Webhook>(
        r#"
        INSERT INTO webhooks (id, collection, event, url, secret)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, collection, event, url, secret, enabled, created_at, updated_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(collection)
    .bind(event)
    .bind(url)
    .bind(secret)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn delete_webhook(pool: &DbPool, id: Uuid) -> Result<(), ApiError> {
    let result = sqlx::query("DELETE FROM webhooks WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }
    Ok(())
}

pub async fn emit_event(pool: DbPool, collection: String, event: WebhookEventKind, content: Value) {
    let hooks = match matching_webhooks(&pool, &collection, event.as_str()).await {
        Ok(hooks) => hooks,
        Err(err) => {
            tracing::error!("webhook lookup failed: {err}");
            return;
        }
    };

    if hooks.is_empty() {
        return;
    }

    let payload = WebhookEventPayload {
        event: event.as_str().to_string(),
        collection: collection.clone(),
        content,
        timestamp: Utc::now().to_rfc3339(),
    };

    for hook in hooks {
        let pool = pool.clone();
        let payload = payload.clone();
        tokio::spawn(async move {
            if let Err(err) = deliver_one(&pool, &hook, &payload).await {
                tracing::error!("webhook delivery failed: {err}");
            }
        });
    }
}

async fn matching_webhooks(pool: &DbPool, collection: &str, event: &str) -> Result<Vec<Webhook>, ApiError> {
    let rows = sqlx::query_as::<_, Webhook>(
        r#"
        SELECT id, collection, event, url, secret, enabled, created_at, updated_at
        FROM webhooks
        WHERE enabled = true
          AND (collection = $1 OR collection = '*')
          AND (event = $2 OR event = '*')
        ORDER BY created_at DESC
        "#,
    )
    .bind(collection)
    .bind(event)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

async fn deliver_one(
    pool: &DbPool,
    hook: &Webhook,
    payload: &WebhookEventPayload,
) -> Result<(), ApiError> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| ApiError::Internal(anyhow::anyhow!("reqwest client build failed: {e}")))?;

    let body = serde_json::to_string(payload)
        .map_err(|e| ApiError::Internal(anyhow::anyhow!("serialize webhook payload failed: {e}")))?;
    let signature = sign_payload(&hook.secret, &body)?;

    let mut attempts = 0;
    let mut last_error: Option<String> = None;
    let mut response_status: Option<i32> = None;

    for delay_ms in [1_000_u64, 2_000, 4_000] {
        attempts += 1;
        match client
            .post(&hook.url)
            .header("X-RAKIT-Event", payload.event.clone())
            .header("X-RAKIT-Collection", payload.collection.clone())
            .header("X-RAKIT-Signature", signature.clone())
            .body(body.clone())
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => {
                response_status = Some(resp.status().as_u16() as i32);
                insert_delivery(pool, hook.id, payload, "delivered", attempts, None, response_status).await?;
                return Ok(());
            }
            Ok(resp) => {
                response_status = Some(resp.status().as_u16() as i32);
                last_error = Some(format!("non-2xx response: {}", resp.status()));
            }
            Err(err) => {
                last_error = Some(err.to_string());
            }
        }

        tokio::time::sleep(Duration::from_millis(delay_ms)).await;
    }

    insert_delivery(
        pool,
        hook.id,
        payload,
        "failed",
        attempts,
        last_error,
        response_status,
    )
    .await?;

    Ok(())
}

async fn insert_delivery(
    pool: &DbPool,
    webhook_id: Uuid,
    payload: &WebhookEventPayload,
    status: &str,
    attempts: i32,
    last_error: Option<String>,
    response_status: Option<i32>,
) -> Result<(), ApiError> {
    sqlx::query(
        r#"
        INSERT INTO webhook_deliveries (id, webhook_id, event, status, attempts, last_error, response_status, payload)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(webhook_id)
    .bind(&payload.event)
    .bind(status)
    .bind(attempts)
    .bind(last_error)
    .bind(response_status)
    .bind(serde_json::to_value(payload).map_err(|e| ApiError::Internal(anyhow::anyhow!(e)))?)
    .execute(pool)
    .await?;
    Ok(())
}

fn sign_payload(secret: &str, body: &str) -> Result<String, ApiError> {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|e| ApiError::Internal(anyhow::anyhow!("invalid webhook secret: {e}")))?;
    mac.update(body.as_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
}

fn validate_webhook_collection(collection: &str) -> Result<(), ApiError> {
    if collection == "*" {
        return Ok(());
    }
    validate_collection_name(collection)
}

fn validate_webhook_event(event: &str) -> Result<(), ApiError> {
    match event {
        "created" | "updated" | "deleted" | "*" => Ok(()),
        _ => Err(ApiError::BadRequest("invalid webhook event".into())),
    }
}

fn validate_webhook_url(url: &str) -> Result<(), ApiError> {
    let parsed = Url::parse(url).map_err(|_| ApiError::BadRequest("invalid webhook url".into()))?;
    match parsed.scheme() {
        "http" | "https" => Ok(()),
        _ => Err(ApiError::BadRequest("webhook url must be http(s)".into())),
    }
}
