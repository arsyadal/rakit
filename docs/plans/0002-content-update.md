# Plan 0002 — Content Update Endpoint (PUT / PATCH)

| Meta | Value |
|---|---|
| **Status** | ✅ Done |
| **Owner** | @you |
| **Depends on** | — |
| **Blocks** | 0003 Collections |
| **Effort** | S |
| **Created** | 2026-04-30 |

---

## 1. 🎯 Goal

Complete CRUD operations for the `contents` resource:
- **PUT** — full replacement of the `data` field
- **PATCH** — deep merge of JSON fields (partial update)
- Both return updated content with new `updated_at` timestamp

## 2. 🚫 Non-goals

- Revision history / content versioning (future)
- Optimistic concurrency control via ETag (future)
- Conflict resolution strategies (last-write-wins for now)

## 3. 🌐 API Contract

### `PUT /api/v1/contents/:id`

Full replacement of `data` field.

**Request:**
```json
{
  "title": "Updated title",
  "body": "New content"
}
```

**Response 200:**
```json
{
  "id": "...",
  "data": {"title": "Updated title", "body": "New content"},
  "created_at": "...",
  "updated_at": "2026-04-30T15:30:00Z"
}
```

**Errors:** 404 if not found

---

### `PATCH /api/v1/contents/:id`

Deep merge — existing fields preserved, new/changed fields updated.

**Request:**
```json
{"published": true}
```

If existing data was `{"title":"Hello","published":false}`, result:
```json
{
  "id": "...",
  "data": {"title": "Hello", "published": true},
  "created_at": "...",
  "updated_at": "2026-04-30T15:31:00Z"
}
```

**Errors:** 404 if not found

## 4. 🗄️ Data Model

No schema changes — reuse existing `contents` table and `updated_at` trigger.

## 5. 📁 File Changes

| File | Change | Purpose |
|---|---|---|
| `src/services/content.rs` | UPDATE | Add `update()` and `patch()` functions |
| `src/api/handlers/content.rs` | UPDATE | Add `update` and `patch` handlers |
| `src/api/routes/content.rs` | UPDATE | Mount PUT & PATCH routes |

## 6. 🛠️ Implementation Steps

1. ✅ Add `update(pool, id, new_data)` to `services/content.rs` — SQL UPDATE
2. ✅ Add `patch(pool, id, partial_data)` — use PostgreSQL `jsonb ||` operator for merge
3. ✅ Add handlers `update` and `patch` in `handlers/content.rs`
4. ✅ Mount routes `.route("/:id", put(update).patch(patch))` in `routes/content.rs`
5. ✅ Manual test with curl

## 7. 🧪 Tests

```bash
# Create a content first
ID=$(curl -s -X POST http://localhost:3000/api/v1/contents \
  -H "Content-Type: application/json" \
  -d '{"title":"Original","count":10}' | jq -r '.id')

# PUT — full replace
curl -X PUT http://localhost:3000/api/v1/contents/$ID \
  -H "Content-Type: application/json" \
  -d '{"title":"Replaced","new_field":"added"}'
# Expected: data = {title:Replaced, new_field:added} (count removed)

# PATCH — partial merge
curl -X PATCH http://localhost:3000/api/v1/contents/$ID \
  -H "Content-Type: application/json" \
  -d '{"status":"published"}'
# Expected: data = {title:Replaced, new_field:added, status:published}
```

## 8. 🔒 Security Considerations

- No auth required yet (will be added when RBAC lands in 0005)
- Validate that `id` is valid UUID to prevent injection
- Deep merge uses PostgreSQL native `||` operator (safe)

## 9. ⚡ Performance Considerations

- **PATCH merge:** use PostgreSQL `jsonb || $1` (server-side) instead of fetch→merge→update in Rust
- Single roundtrip UPDATE query for both PUT and PATCH

## 10. ❓ Open Questions

- [x] **PATCH semantics?** → Shallow merge via PostgreSQL `||` (top-level keys only). Deep merge in future if needed.
- [x] **ETag / If-Match?** → Not implemented in v1. Simple last-write-wins.

## 11. 📚 References

- [RFC 7396 — JSON Merge Patch](https://www.rfc-editor.org/rfc/rfc7396)
- [RFC 6902 — JSON Patch](https://www.rfc-editor.org/rfc/rfc6902)
