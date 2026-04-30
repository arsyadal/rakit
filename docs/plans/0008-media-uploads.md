# Plan 0008 — Media / File Uploads

| Meta | Value |
|---|---|
| **Status** | 🟡 Draft (skeleton) |
| **Owner** | @you |
| **Depends on** | 0001 |
| **Blocks** | — |
| **Effort** | L |
| **Created** | 2026-04-30 |

---

## 1. 🎯 Goal

_TBD — Allow uploading binary assets (images, PDFs, etc.) to S3-compatible
storage and reference them from content entries._

## 2. 🚫 Non-goals

_TBD — image transformation pipeline (resize/crop) — future plan._

## 3. 🌐 API Contract

_TBD — `POST /api/v1/_media` (multipart), `GET /api/v1/_media/:id`,
or presigned URL strategy._

## 4. 🗄️ Data Model

_TBD — `media_assets` table: id, filename, mime, size, storage_key, owner_id._

## 5. 📁 File Changes

_TBD._

## 6. 🛠️ Implementation Steps

_TBD — pick S3 crate (aws-sdk-s3 or rust-s3 or object_store)._

## 7. 🧪 Tests

_TBD — use MinIO in docker-compose for integration tests._

## 8. 🔒 Security Considerations

_TBD — MIME sniffing, size limits, virus scan hook, signed URLs._

## 9. ⚡ Performance Considerations

_TBD — streaming uploads (don't buffer entire file in memory)._

## 10. ❓ Open Questions

- [ ] Direct upload (presigned URL) vs proxy through RAKIT?
- [ ] Where to store: local FS (dev) + S3 (prod) abstraction?
- [ ] Public vs private buckets?

## 11. 📚 References

- [object_store crate](https://crates.io/crates/object_store)
- [MinIO docker setup](https://min.io/docs/minio/container/index.html)
