# Plan 0007 — Webhooks

| Meta | Value |
|---|---|
| **Status** | 🟡 Draft (skeleton) |
| **Owner** | @you |
| **Depends on** | 0003 |
| **Blocks** | — |
| **Effort** | M |
| **Created** | 2026-04-30 |

---

## 1. 🎯 Goal

_TBD — Outbound HTTP callbacks when content is created / updated / deleted,
with retries and HMAC signing._

## 2. 🚫 Non-goals

_TBD — inbound webhooks (different feature)._

## 3. 🌐 API Contract

_TBD — `POST /api/v1/_webhooks` to register, payload format spec._

## 4. 🗄️ Data Model

_TBD — `webhooks`, `webhook_deliveries` tables._

## 5. 📁 File Changes

_TBD._

## 6. 🛠️ Implementation Steps

_TBD — background worker (tokio task) that pulls from delivery queue._

## 7. 🧪 Tests

_TBD — mock HTTP server, simulate failures._

## 8. 🔒 Security Considerations

_TBD — HMAC-SHA256 signature header, SSRF protection (deny private IPs)._

## 9. ⚡ Performance Considerations

_TBD — non-blocking dispatch, exponential backoff, max attempts._

## 10. ❓ Open Questions

- [ ] At-least-once vs exactly-once delivery?
- [ ] Per-webhook secret or global?
- [ ] Delivery log retention period?

## 11. 📚 References

- [Stripe webhooks design](https://stripe.com/docs/webhooks)
- [GitHub webhooks security](https://docs.github.com/en/webhooks/using-webhooks/validating-webhook-deliveries)
