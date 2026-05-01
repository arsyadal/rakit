# 🗺️ RAKIT Roadmap

> Helicopter view of all planned features. Each item links to a detailed plan in [`docs/plans/`](./plans/).
>
> **Legend:** 🟡 Draft · 🟢 Ready to execute · 🔵 In progress · ✅ Done · ⏸️ Paused

---

## 🎯 Vision

A lightweight, high-performance **headless CMS engine** in Rust — competitive
with Strapi/Directus on features, but with a fraction of the memory footprint
and orders of magnitude better throughput.

---

## 📍 Current State (MVP — done)

- ✅ Axum HTTP server + tracing
- ✅ PostgreSQL pool via SQLx + auto migrations
- ✅ Generic `contents` table (jsonb) — dynamic content storage
- ✅ Endpoints: `POST /contents`, `GET /contents`, `GET /contents/:id`, `DELETE /contents/:id`
- ✅ Unified `ApiError` → HTTP response
- ✅ Docker Compose for local Postgres
- ✅ CI workflow (fmt, clippy, build, test)

---

## 🚧 Backlog (ordered by recommended execution sequence)

| # | Plan | Status | Depends on | Effort | Notes |
|---|------|--------|------------|--------|-------|
| 0001 | [Auth — JWT + Argon2](./plans/0001-auth-jwt.md) | ✅ Done | — | M | Foundation for every protected endpoint |
| 0002 | [Content Update endpoint (PUT/PATCH)](./plans/0002-content-update.md) | ✅ Done | — | S | Completes CRUD |
| 0003 | [Collections / Content Types](./plans/0003-collections.md) | ✅ Done | 0002 | M | Namespacing for `posts`, `products`, etc. |
| 0004 | [Query & Filter API](./plans/0004-query-filter.md) | ✅ Done | 0003 | M | jsonb GIN-powered filtering |
| 0005 | [RBAC — Roles & Permissions](./plans/0005-rbac.md) | ✅ Done | 0001, 0003 | L | Per-collection access control |
| 0006 | [Schema Validation per Collection](./plans/0006-schema-validation.md) | ✅ Done | 0003 | M | Optional JSON Schema enforcement |
| 0007 | [Webhooks](./plans/0007-webhooks.md) | ✅ Done | 0003 | M | Outbound events on mutation |
| 0008 | [Media / File Uploads](./plans/0008-media-uploads.md) | 🟡 Draft | 0001 | L | S3-compatible storage |

---

## 🧭 Dependency Graph

```
            ┌──────────────────┐
            │  MVP (✅ done)   │
            └────────┬─────────┘
                     │
        ┌────────────┼────────────┐
        ▼            ▼            ▼
     0001 Auth    0002 Update   0008 Media (needs 0001)
        │            │
        │            ▼
        │         0003 Collections
        │            │
        │   ┌────────┼─────────┐
        │   ▼        ▼         ▼
        │ 0004    0006       0007
        │ Query   Schema     Webhooks
        │   │
        ▼   ▼
       0005 RBAC
```

---

## 🔭 Beyond v1.0 (later)

- 🌐 GraphQL endpoint (alongside REST)
- 🔌 Plugin system (WASM-based)
- 🌍 i18n / localized fields
- 📊 Admin dashboard (separate repo)
- 📦 Official JS/TS SDK (`@rakit/client`)
- 🚀 Realtime subscriptions (SSE/WebSocket)

---

## 📋 How to use this roadmap

1. Pick the next 🟡 Draft item with no pending dependencies.
2. Switch to **Opus-thinking** model → ask it to fill the plan file in detail.
3. Mark plan status as 🟢 Ready when reviewed.
4. Switch to **Sonnet** → execute the plan.
5. Mark ✅ Done here once merged.
