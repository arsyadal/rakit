# Plan 0007 — Webhooks

| Meta | Value |
|---|---|
| **Status** | ✅ Done |
| **Owner** | @you |
| **Depends on** | 0003 |
| **Blocks** | — |
| **Effort** | M |
| **Created** | 2026-04-30 |

---

## 1. 🎯 Goal

Add outbound webhooks for content mutations:
- fire on `created`, `updated`, and `deleted`
- sign payloads with HMAC-SHA256
- deliver asynchronously with retries
- keep the write path non-blocking

## 2. 🚫 Non-goals

- Inbound webhooks
- Full workflow orchestration
- Dead-letter queue UI
- Advanced routing expressions

## 3. 🌐 API Contract

### `POST /api/v1/_webhooks`
Register a webhook.

```json
{
  "collection": "posts",
  "event": "created",
  "url": "https://example.com/webhooks/rakit",
  "secret": "super-secret"
}
```

### `GET /api/v1/_webhooks`
List registered webhooks.

### `DELETE /api/v1/_webhooks/:id`
Delete a webhook.

### Delivery payload
```json
{
  "event": "created",
  "collection": "posts",
  "content": { ... },
  "timestamp": "2026-05-01T00:00:00Z"
}
```

Headers:
- `X-RAKIT-Event`
- `X-RAKIT-Collection`
- `X-RAKIT-Signature`

## 4. 🗄️ Data Model

### `webhooks`
- id
- collection (`*` allowed)
- event (`created` / `updated` / `deleted` / `*`)
- url
- secret
- enabled
- created_at / updated_at

### `webhook_deliveries`
- id
- webhook_id
- event
- status (`pending` / `delivered` / `failed`)
- attempts
- last_error
- response_status
- created_at / updated_at

## 5. 📁 File Changes

| File | Change | Purpose |
|---|---|---|
| `migrations/20260430000005_webhooks.sql` | NEW | Webhooks tables |
| `src/models/webhook.rs` | NEW | Webhook DTOs |
| `src/models/mod.rs` | UPDATE | Export webhook models |
| `src/services/webhook.rs` | NEW | Registry + dispatch |
| `src/services/content.rs` | UPDATE | Emit events after writes |
| `src/api/handlers/webhook.rs` | NEW | Webhook CRUD endpoints |
| `src/api/handlers/mod.rs` | UPDATE | Export webhook handlers |
| `src/api/routes/mod.rs` | UPDATE | Add `/_webhooks` routes |
| `Cargo.toml` | UPDATE | reqwest/hmac/sha2/hex deps |

## 6. 🛠️ Implementation Steps

1. Add webhook + delivery tables
2. Add webhook registry models and CRUD service
3. Add HMAC signing + HTTP delivery helper
4. Hook content mutations to emit events asynchronously
5. Add admin-only webhook CRUD endpoints
6. Test: success, retry, and failure paths

## 7. 🧪 Tests

- Register webhook and verify it appears in list
- Create/update/delete content and verify callback server receives payload
- Simulate non-2xx responses to ensure retries occur
- Verify signature header is present and valid

## 8. 🔒 Security Considerations

- HMAC-SHA256 signing with per-webhook secret
- Reject non-HTTP(S) URLs
- Avoid obvious private-network destinations where possible
- Fail closed on signing errors

## 9. ⚡ Performance Considerations

- Delivery is async so content writes stay fast
- Exponential backoff: 1s, 2s, 4s (max 3 attempts)
- Delivery logs stored for observability

## 10. ❓ Open Questions

- [x] At-least-once vs exactly-once delivery? → At-least-once
- [x] Per-webhook secret or global? → Per-webhook secret
- [x] Delivery log retention period? → Keep logs indefinitely in v1

## 11. 📚 References

- [Stripe webhooks design](https://stripe.com/docs/webhooks)
- [GitHub webhooks security](https://docs.github.com/en/webhooks/using-webhooks/validating-webhook-deliveries)
