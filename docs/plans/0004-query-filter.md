# Plan 0004 — Query & Filter API

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

Provide a safe query-string API for collection listings:
- filter content using simple `where` expressions
- sort by metadata or top-level json fields
- paginate with `limit` + `offset`
- keep all SQL parameterized (no string concatenation of user values)

## 2. 🚫 Non-goals

- Full-text search (later)
- Aggregations / grouping (later)
- Deep nested json path queries (later)
- OR expressions / complex boolean logic (later)

## 3. 🌐 API Contract

### Chosen syntax (minimal & safe)

- `where=field:op:value,field:op:value` (comma-separated)
- `sort=-created_at` or `sort=created_at.desc`
- `limit=20`
- `offset=0`

### Examples

```bash
GET /api/v1/posts?where=published:eq:true,title:contains:Rust&sort=-created_at&limit=20
GET /api/v1/products?where=data.price:eq:100&sort=data.price.desc
```

### Supported operators

- `eq`, `ne`
- `contains`
- `gt`, `gte`, `lt`, `lte` (metadata/date fields; simple text comparison for json fields is not enabled in v1)

### Response

Same JSON shape as the list endpoint today, but filtered/sorted/paginated.

## 4. 🗄️ Data Model

No schema change expected. Uses existing `contents` table, the `collection`
column, and the existing `jsonb` `data` column plus indexes.

## 5. 📁 File Changes

| File | Change | Purpose |
|---|---|---|
| `src/api/handlers/content.rs` | UPDATE | Parse query params for list endpoint |
| `src/services/content.rs` | UPDATE | Build safe dynamic SQL with `QueryBuilder` |
| `src/utils/mod.rs` | UPDATE | Add query validation helpers |
| `docs/ROADMAP.md` | UPDATE | Mark 0004 done |

## 6. 🛠️ Implementation Steps

1. Parse query parameters into a `ListQuery` struct
2. Validate field names with a strict allowlist regex
3. Parse `where` expressions into column/operator/value triples
4. Build the SQL with `sqlx::QueryBuilder<Postgres>`
5. Keep `collection = $1` as the base constraint
6. Add `ORDER BY`, `LIMIT`, `OFFSET`
7. Test normal queries and adversarial inputs

## 7. 🧪 Tests

```bash
# Filter by boolean
curl 'http://localhost:3000/api/v1/posts?where=published:eq:true'

# Sort descending and paginate
curl 'http://localhost:3000/api/v1/posts?sort=-created_at&limit=10&offset=0'

# Filter by title substring
curl 'http://localhost:3000/api/v1/posts?where=title:contains:Rust'

# Injection attempt should fail validation
curl 'http://localhost:3000/api/v1/posts?where=title:eq:abc%27%20OR%201=1--'
```

## 8. 🔒 Security Considerations

- Strict allowlist of operators
- Field names validated with regex before being embedded into SQL
- All values bound as parameters
- Max page size = 100
- No nested path traversal beyond one `data.foo` level
- No raw SQL fragments from user input

## 9. ⚡ Performance Considerations

- `collection` filter uses composite index
- `jsonb` GIN index still helps for future expansions
- `LIMIT` defaults to a small number to keep responses cheap
- Consider `EXPLAIN ANALYZE` logging in dev for future tuning

## 10. ❓ Open Questions

- [x] **Syntax?** → Custom minimal `where=field:op:value`
- [x] **Cursor or offset?** → Offset for v1 (simple and predictable)
- [x] **Max complexity?** → One-level field paths only; no OR logic

## 11. 📚 References

- [PostgREST operators](https://postgrest.org/en/stable/api.html#operators)
- [PostgreSQL jsonb operators](https://www.postgresql.org/docs/current/functions-json.html)
