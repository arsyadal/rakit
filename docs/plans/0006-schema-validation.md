# Plan 0006 — Schema Validation per Collection

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

_TBD — Optional JSON Schema enforcement per collection, so producers get
typed validation while keeping the dynamic jsonb storage._

## 2. 🚫 Non-goals

_TBD — UI for schema editing (future, separate admin repo)._

## 3. 🌐 API Contract

_TBD — `PUT /api/v1/_schemas/:collection` to register/update schema._

## 4. 🗄️ Data Model

_TBD — `collection_schemas` table: collection, json_schema, version._

## 5. 📁 File Changes

_TBD._

## 6. 🛠️ Implementation Steps

_TBD — pick a Rust JSON Schema validator (jsonschema crate)._

## 7. 🧪 Tests

_TBD._

## 8. 🔒 Security Considerations

_TBD — guard against pathological schemas (regex DoS)._

## 9. ⚡ Performance Considerations

_TBD — compile schema once, reuse compiled validator (LRU cache)._

## 10. ❓ Open Questions

- [ ] JSON Schema draft 2020-12 only, or older drafts too?
- [ ] What to do with existing rows when schema changes (migrate? warn? ignore)?
- [ ] Strict vs warn-only mode?

## 11. 📚 References

- [JSON Schema 2020-12](https://json-schema.org/specification.html)
- [`jsonschema` crate](https://crates.io/crates/jsonschema)
