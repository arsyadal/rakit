# Plan 0003 — Collections / Content Types

| Meta | Value |
|---|---|
| **Status** | 🟡 Draft (skeleton) |
| **Owner** | @you |
| **Depends on** | 0002 |
| **Blocks** | 0004, 0005, 0006, 0007 |
| **Effort** | M |
| **Created** | 2026-04-30 |

---

## 1. 🎯 Goal

_TBD — Introduce the concept of a "collection" (a.k.a. content type) so users
can group entries: `posts`, `products`, `pages`, etc. Routes become
`/api/v1/:collection/...`._

## 2. 🚫 Non-goals

_TBD — schema validation per collection is plan 0006, not here._

## 3. 🌐 API Contract

_TBD — `POST /api/v1/:collection`, `GET /api/v1/:collection`, etc._

## 4. 🗄️ Data Model

_TBD — add `collection TEXT NOT NULL` column + index, or introduce a
`collections` registry table. Decision pending._

## 5. 📁 File Changes

_TBD._

## 6. 🛠️ Implementation Steps

_TBD._

## 7. 🧪 Tests

_TBD._

## 8. 🔒 Security Considerations

_TBD — sanitize collection name (regex `^[a-z][a-z0-9_]{1,40}$`)._

## 9. ⚡ Performance Considerations

_TBD — composite index on `(collection, created_at DESC)`._

## 10. ❓ Open Questions

- [ ] Single table with `collection` column, or one table per collection?
- [ ] Should collections be auto-created on first POST, or pre-registered?
- [ ] Reserved collection names (`_users`, `_system`)?

## 11. 📚 References

- Strapi: "Content Types"
- Directus: "Collections"
- Supabase: tables-as-collections
