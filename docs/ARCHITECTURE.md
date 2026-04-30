# RAKIT Architecture

## Layers

```
HTTP  →  handlers  →  services  →  db (sqlx)
                       ↑
                    models / errors
```

- **handlers**: parse/validate input, return JSON. No SQL here.
- **services**: business rules, transactions, orchestration.
- **models**: typed rows / DTOs shared across layers.
- **errors**: single `ApiError` enum → HTTP response.

## The Core Idea

RAKIT stores user content in a single `contents` table with a `jsonb` `data`
column. This means **adding a new content type does not require a migration** —
clients simply POST a new JSON shape. Indexing is handled via a GIN index on
`data`, with optional typed projections added later as needed.

## Next Milestones

1. **Auth** — JWT + Argon2, `users` table, `/auth/login` & `/auth/register`.
2. **Schemas** — optional registered schemas to validate `data` shapes.
3. **RBAC** — per-collection permissions.
4. **Webhooks** — outbound events on content mutation.
