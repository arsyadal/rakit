# Plan 0003 — Collections / Content Types

| Meta | Value |
|---|---|
| **Status** | ✅ Done |
| **Owner** | @you |
| **Depends on** | 0002 |
| **Blocks** | 0004, 0005, 0006, 0007 |
| **Effort** | M |
| **Created** | 2026-04-30 |

---

## 1. 🎯 Goal

Introduce **collections** (content types) to namespace content:
- API routes become `/api/v1/:collection` (e.g., `/api/v1/posts`, `/api/v1/products`)
- Collection name stored in `collection` column for filtering
- Validation: collection names must match `^[a-z][a-z0-9_]{0,39}$`

## 2. 🚫 Non-goals

- Schema validation per collection (plan 0006)
- Collection metadata / descriptions (future)
- Collection-level settings (future)

## 3. 🌐 API Contract

All existing content endpoints now namespaced by collection:

- `POST /api/v1/:collection` — create in collection
- `GET /api/v1/:collection` — list from collection
- `GET /api/v1/:collection/:id` — get one
- `PUT /api/v1/:collection/:id` — update
- `PATCH /api/v1/:collection/:id` — patch
- `DELETE /api/v1/:collection/:id` — delete

**Example:**
```bash
POST /api/v1/posts -d '{"title":"Hello"}'
GET /api/v1/posts
GET /api/v1/products
```

Collections are **auto-created** on first POST (no pre-registration needed).

## 4. 🗄️ Data Model

### Alter `contents` table

```sql
ALTER TABLE contents ADD COLUMN collection TEXT NOT NULL DEFAULT 'default';
CREATE INDEX idx_contents_collection ON contents (collection, created_at DESC);
```

**Decision:** Single table with `collection` column (not one-table-per-collection).

**Note:** Existing rows get `collection = 'default'` for backward compat.

## 5. 📁 File Changes

| File | Change | Purpose |
|---|---|---|
| `migrations/20260430000002_collections.sql` | NEW | Add collection column + index |
| `src/models/content.rs` | UPDATE | Add `collection` field |
| `src/services/content.rs` | UPDATE | Filter by collection in all queries |
| `src/api/handlers/content.rs` | UPDATE | Extract collection from path |
| `src/api/routes/mod.rs` | UPDATE | Change route structure to `/:collection` |
| `src/api/routes/content.rs` | DELETE | Merge into mod.rs (simpler) |
| `src/utils/mod.rs` | UPDATE | Add collection name validator |

## 6. 🛠️ Implementation Steps

1. ✅ Write migration to add `collection` column
2. ✅ Update `Content` model to include `collection: String`
3. ✅ Add `validate_collection_name(name)` utility
4. ✅ Update all service functions to accept & filter by `collection`
5. ✅ Update handlers to extract `collection` from path param
6. ✅ Restructure routes: `/api/v1/:collection` instead of `/api/v1/contents`
7. ✅ Test with multiple collections

## 7. 🧪 Tests

```bash
# Create in 'posts' collection
curl -X POST http://localhost:3000/api/v1/posts \
  -H "Content-Type: application/json" \
  -d '{"title":"My First Post"}'

# Create in 'products' collection
curl -X POST http://localhost:3000/api/v1/products \
  -H "Content-Type: application/json" \
  -d '{"name":"Widget","price":99}'

# List posts (should not include products)
curl http://localhost:3000/api/v1/posts

# List products
curl http://localhost:3000/api/v1/products

# Invalid collection name (expect 400)
curl -X POST http://localhost:3000/api/v1/Invalid-Name \
  -d '{}'
```

## 8. 🔒 Security Considerations

- **Strict regex:** `^[a-z][a-z0-9_]{0,39}$` (lowercase, alphanumeric + underscore, max 40 chars)
- **Reserved names:** block `_internal`, `_system`, `_admin`, `_meta`
- **SQL injection:** collection name validated before query (never interpolated)

## 9. ⚡ Performance Considerations

- Composite index `(collection, created_at DESC)` ensures fast filtered listing
- GIN index on `data` still works across collections

## 10. ❓ Open Questions

- [x] **Single table vs per-collection tables?** → Single table (simpler schema, easier to query across collections in future)
- [x] **Auto-create or pre-register?** → Auto-create (better DX, less ceremony)
- [x] **Reserved names?** → Block `_*` prefix for system use

## 11. 📚 References

- Strapi: "Content Types"
- Directus: "Collections"
- Supabase: tables-as-collections
