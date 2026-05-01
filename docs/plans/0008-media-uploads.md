# Plan 0008 — Media / File Uploads

| Meta | Value |
|---|---|
| **Status** | ✅ Done |
| **Owner** | @you |
| **Depends on** | 0001 |
| **Blocks** | — |
| **Effort** | L |
| **Created** | 2026-04-30 |

---

## 1. 🎯 Goal

Allow authenticated users to upload binary assets (images, PDFs, etc.) and store them as reusable media objects. RAKIT will persist metadata in Postgres and file bytes on local filesystem for v1, with a storage abstraction that can later be swapped to S3-compatible backends.

## 2. 🚫 Non-goals

- Image transformation pipeline (resize/crop)
- Virus scanning pipeline
- Direct-to-S3 presigned upload flow
- Public CDN/asset optimization
- Chunked/resumable uploads

## 3. 🌐 API Contract

### `POST /api/v1/_media`
Multipart upload with field name `file`.

### `GET /api/v1/_media`
List current user's uploads.

### `GET /api/v1/_media/:id`
Return media metadata.

### `GET /api/v1/_media/:id/download`
Stream the binary file.

### `DELETE /api/v1/_media/:id`
Delete media owned by current user (or admin).

## 4. 🗄️ Data Model

### `media_assets`
- id
- owner_id (FK users.id)
- filename
- mime_type
- size_bytes
- storage_key
- storage_backend (`local` for v1)
- created_at / updated_at

Files are stored under `./uploads/<storage_key>`.

## 5. 📁 File Changes

| File | Change | Purpose |
|---|---|---|
| `migrations/20260430000006_media.sql` | NEW | Media table |
| `src/models/media.rs` | NEW | Media DTO |
| `src/models/mod.rs` | UPDATE | Export media model |
| `src/services/media.rs` | NEW | Storage + metadata CRUD |
| `src/api/handlers/media.rs` | NEW | Upload/list/get/download/delete |
| `src/api/handlers/mod.rs` | UPDATE | Export media handlers |
| `src/api/routes/mod.rs` | UPDATE | Add `/_media` routes |
| `Cargo.toml` | UPDATE | Enable axum multipart |
| `README.md` | UPDATE | Document uploads usage |_

## 6. 🛠️ Implementation Steps

1. Add media table migration
2. Enable axum multipart support
3. Add media model + service CRUD
4. Save uploads to `./uploads` with UUID storage keys
5. Expose metadata and download endpoints
6. Protect routes with JWT auth and ownership checks
7. Test upload/download/delete flows

## 7. 🧪 Tests

- Upload a file with multipart form data
- Fetch metadata
- Download binary and verify checksum/size
- Delete file and verify metadata/file are gone
- Unauthorized delete/list should fail

## 8. 🔒 Security Considerations

- Max upload size enforced in handler
- MIME type comes from client but is stored as metadata only
- Ownership checks on access/delete
- Avoid path traversal by using UUID storage keys only

## 9. ⚡ Performance Considerations

- Multipart upload is buffered in memory for v1 (small/medium assets only)
- Future improvement: streaming writer + object storage backend
- Download streams directly from file path

## 10. ❓ Open Questions

- [x] Direct upload (presigned URL) vs proxy through RAKIT? → proxy through RAKIT for v1
- [x] Where to store: local FS (dev) + S3 (prod) abstraction? → local FS now, abstract later
- [x] Public vs private buckets? → private by default; download endpoint is authenticated

## 11. 📚 References

- [object_store crate](https://crates.io/crates/object_store)
- [MinIO docker setup](https://min.io/docs/minio/container/index.html)
