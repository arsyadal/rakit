# Plan 0006 — Schema Validation per Collection

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

Add optional per-collection JSON Schema validation while keeping RAKIT's
`jsonb` storage model. Producers can register a schema for a collection,
and all future create/update/patch operations for that collection are validated
before data is persisted.

## 2. 🚫 Non-goals

- Schema editing UI (future admin app)
- Automatic migrations of existing content when a schema changes
- Complex schema versioning history beyond a simple version counter
- Non-JSON payload types

## 3. 🌐 API Contract

### `PUT /api/v1/_schemas/:collection`
Register or replace the JSON Schema for a collection.

**Request:**
```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "required": ["title"],
  "properties": {
    "title": { "type": "string" },
    "published": { "type": "boolean" }
  },
  "additionalProperties": true
}
```

**Response 200:**
```json
{ "collection": "posts", "version": 1 }
```

### `GET /api/v1/_schemas/:collection`
Return the stored schema (or 404 if none exists).

**Validation behavior:**
- `POST /api/v1/:collection`
- `PUT /api/v1/:collection/:id`
- `PATCH /api/v1/:collection/:id`

All validate against the collection schema if one is registered.
Invalid payloads return `400` with a schema validation error message.

## 4. 🗄️ Data Model

### `collection_schemas`

```sql
CREATE TABLE collection_schemas (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    collection  TEXT NOT NULL UNIQUE,
    schema_json JSONB NOT NULL,
    version     INTEGER NOT NULL DEFAULT 1,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

- One schema per collection
- `version` increments on update
- Stored as `jsonb` to preserve the full schema document

## 5. 📁 File Changes

| File | Change | Purpose |
|---|---|---|
| `migrations/20260430000004_schema_validation.sql` | NEW | Schema table |
| `src/models/schema.rs` | NEW | Schema DTOs |
| `src/models/mod.rs` | UPDATE | Export schema module |
| `src/services/schema.rs` | NEW | Schema CRUD + validation helpers |
| `src/services/content.rs` | UPDATE | Validate before persist |
| `src/api/handlers/schema.rs` | NEW | Schema endpoints |
| `src/api/handlers/mod.rs` | UPDATE | Export schema handlers |
| `src/api/routes/mod.rs` | UPDATE | Add `/_schemas` routes |
| `src/utils/mod.rs` | UPDATE | Add JSON Schema helpers |
| `Cargo.toml` | UPDATE | Add `jsonschema` dependency |

## 6. 🛠️ Implementation Steps

1. Add `collection_schemas` migration
2. Add schema DTOs + service helpers
3. Add `jsonschema` dependency
4. Create `PUT/GET /api/v1/_schemas/:collection`
5. Validate schema document itself when saving it
6. Validate content payloads on create/update/patch
7. Add tests for valid/invalid payloads

## 7. 🧪 Tests

```bash
# Register schema for posts
curl -X PUT http://localhost:3000/api/v1/_schemas/posts \
  -H "Authorization: Bearer <admin-token>" \
  -H "Content-Type: application/json" \
  -d '{"type":"object","required":["title"],"properties":{"title":{"type":"string"},"published":{"type":"boolean"}},"additionalProperties":true}'

# Valid payload
curl -X POST http://localhost:3000/api/v1/posts \
  -H "Content-Type: application/json" \
  -d '{"title":"Hello"}'

# Invalid payload (missing title)
curl -X POST http://localhost:3000/api/v1/posts \
  -H "Content-Type: application/json" \
  -d '{"published":true}'
# expect 400
```

## 8. 🔒 Security Considerations

- Validate schema input before saving
- Use the schema validator library to avoid hand-rolled evaluation
- Keep `additionalProperties` allowed by default unless explicitly disabled
- No arbitrary code execution; JSON Schema only

## 9. ⚡ Performance Considerations

- Compile schema only when saving or validating content
- Optional later optimization: cache compiled validators per collection
- Validation happens only when a schema exists

## 10. ❓ Open Questions

- [x] JSON Schema draft 2020-12 only (crate default)
- [x] Existing rows remain unchanged; new writes are validated
- [x] Strict mode only (reject invalid payloads)

## 11. 📚 References

- [JSON Schema 2020-12](https://json-schema.org/specification.html)
- [`jsonschema` crate](https://crates.io/crates/jsonschema)
