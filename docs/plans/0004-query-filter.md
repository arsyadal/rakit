# Plan 0004 — Query & Filter API

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

_TBD — Provide a safe, expressive query string API to filter, sort, and
paginate jsonb content using the existing GIN index._

## 2. 🚫 Non-goals

_TBD — full-text search (later), aggregations (later)._

## 3. 🌐 API Contract

_TBD — pick a syntax. Candidates:_
- _PostgREST-style: `?data->>status=eq.published&order=created_at.desc&limit=20`_
- _Strapi-style: `?filters[status][$eq]=published&sort=createdAt:desc`_
- _Custom minimal._

## 4. 🗄️ Data Model

_No schema change expected._

## 5. 📁 File Changes

_TBD._

## 6. 🛠️ Implementation Steps

_TBD — write a small parser → builds parameterized SQL (NEVER concat user input)._

## 7. 🧪 Tests

_TBD — heavy on adversarial inputs (SQL injection attempts)._

## 8. 🔒 Security Considerations

_TBD — strict allowlist of operators, parameterized queries only,
limit max page size, deny deeply nested paths._

## 9. ⚡ Performance Considerations

_TBD — make sure queries hit GIN index; add EXPLAIN ANALYZE in dev mode._

## 10. ❓ Open Questions

- [ ] Adopt PostgREST syntax (familiar to many devs)?
- [ ] Cursor pagination vs offset?
- [ ] Maximum query complexity / depth?

## 11. 📚 References

- [PostgREST operators](https://postgrest.org/en/stable/api.html#operators)
- [PostgreSQL jsonb operators](https://www.postgresql.org/docs/current/functions-json.html)
