# Plan NNNN — <Feature Name>

> Copy this file to `NNNN-feature-name.md` when creating a new plan.

| Meta | Value |
|---|---|
| **Status** | 🟡 Draft |
| **Owner** | @you |
| **Depends on** | — |
| **Blocks** | — |
| **Effort** | S / M / L |
| **Created** | YYYY-MM-DD |

---

## 1. 🎯 Goal

_One or two sentences: what business/technical outcome does this feature deliver?_

## 2. 🚫 Non-goals

_Explicitly list what is OUT of scope to keep the plan focused._

- …
- …

## 3. 🌐 API Contract

### Endpoints

| Method | Path | Auth | Description |
|---|---|---|---|
| POST | `/api/v1/...` | — | … |

### Request / Response shapes

```jsonc
// POST /api/v1/...
// Request
{ }
// Response 200
{ }
// Errors: 400, 401, 404, 409, 500
```

## 4. 🗄️ Data Model

### New tables / columns

```sql
-- migrations/YYYYMMDDHHMMSS_<name>.sql
```

### Indexes

- …

## 5. 📁 File Changes

| File | Change | Purpose |
|---|---|---|
| `src/models/xxx.rs` | NEW | … |
| `src/services/xxx.rs` | NEW | … |
| `src/api/handlers/xxx.rs` | NEW | … |
| `src/api/routes/mod.rs` | UPDATE | register routes |
| `migrations/xxx.sql` | NEW | schema |

## 6. 🛠️ Implementation Steps

1. …
2. …
3. …

## 7. 🧪 Tests

- [ ] Unit: …
- [ ] Integration: …
- [ ] Manual curl verification: …

## 8. 🔒 Security Considerations

- …

## 9. ⚡ Performance Considerations

- …

## 10. ❓ Open Questions

- [ ] …

## 11. 📚 References

- …
