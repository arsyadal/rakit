# Plan 0002 — Content Update Endpoint (PUT / PATCH)

| Meta | Value |
|---|---|
| **Status** | 🟡 Draft (skeleton) |
| **Owner** | @you |
| **Depends on** | — |
| **Blocks** | 0003 Collections |
| **Effort** | S |
| **Created** | 2026-04-30 |

---

## 1. 🎯 Goal

_TBD — Complete the CRUD by adding full-replace (PUT) and partial-merge (PATCH)
update endpoints for the `contents` resource._

## 2. 🚫 Non-goals

_TBD — e.g. revision history / versioning is out of scope here._

## 3. 🌐 API Contract

_TBD — `PUT /api/v1/contents/:id` (replace `data`), `PATCH /api/v1/contents/:id` (deep merge)._

## 4. 🗄️ Data Model

_TBD — likely no schema change; rely on existing `updated_at` trigger._

## 5. 📁 File Changes

_TBD._

## 6. 🛠️ Implementation Steps

_TBD._

## 7. 🧪 Tests

_TBD._

## 8. 🔒 Security Considerations

_TBD — once auth (0001) lands, gate behind middleware._

## 9. ⚡ Performance Considerations

_TBD — PATCH merge strategy: server-side jsonb `||` operator vs Rust-side merge._

## 10. ❓ Open Questions

- [ ] PATCH semantics: shallow merge or deep merge (RFC 7396 JSON Merge Patch)?
- [ ] Use `If-Match` / ETag for optimistic concurrency?

## 11. 📚 References

- [RFC 7396 — JSON Merge Patch](https://www.rfc-editor.org/rfc/rfc7396)
- [RFC 6902 — JSON Patch](https://www.rfc-editor.org/rfc/rfc6902)
