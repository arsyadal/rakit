<div align="center">

# ⚙️ RAKIT

**A lightweight, high-performance headless CMS engine built for the modern web.**

`Fast` · `Minimalist` · `Reliable`

[![CI](https://github.com/arsyadal/rakit/workflows/CI/badge.svg)](https://github.com/arsyadal/rakit/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

</div>

---

## Why RAKIT?

- 🚀 **Blazing Fast** — Powered by Rust (Axum + SQLx).
- 🪶 **Low Footprint** — Runs on **< 20 MB RAM**.
- 🧑‍💻 **Developer First** — Simple JSON API, no bloat.
- 🧩 **Dynamic by Default** — Stores any content shape in PostgreSQL `jsonb`.
- 🔐 **Secure** — JWT authentication with Argon2 password hashing.
- 📦 **Collection-based** — Multi-tenant content namespacing.

## Why the name?

> "Rakit" is an Indonesian word meaning *'to assemble'* or *'to build'*.
> It reflects our mission to provide a platform where developers can
> easily **assemble their content architecture**.

---

## 🚀 Quickstart

```bash
# 1. Spin up Postgres
docker compose up -d

# 2. Configure env
cp .env.example .env
# Edit .env if needed (JWT_SECRET, DATABASE_URL, etc.)

# 3. Run
cargo run
```

The server boots on `http://localhost:3000`.

### Try it

```bash
# 1. Register a user
curl -X POST http://localhost:3000/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"SecurePass123!"}'

# 2. Login (get JWT token)
TOKEN=$(curl -s -X POST http://localhost:3000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"SecurePass123!"}' | jq -r '.token')

# 3. Get current user (protected endpoint)
curl http://localhost:3000/api/v1/auth/me \
  -H "Authorization: Bearer $TOKEN"

# 4. Create content in 'posts' collection
curl -X POST http://localhost:3000/api/v1/posts \
  -H "Content-Type: application/json" \
  -d '{"title":"Hello RAKIT","tags":["intro","rust"],"published":true}'

# 5. List posts
curl http://localhost:3000/api/v1/posts

# 6. Create in different collection
curl -X POST http://localhost:3000/api/v1/products \
  -H "Content-Type: application/json" \
  -d '{"name":"Widget","price":99,"sku":"WDG-001"}'

# 7. Update content (PUT = full replace)
curl -X PUT http://localhost:3000/api/v1/posts/<id> \
  -H "Content-Type: application/json" \
  -d '{"title":"Updated","content":"New content"}'

# 8. Partial update (PATCH = merge)
curl -X PATCH http://localhost:3000/api/v1/posts/<id> \
  -H "Content-Type: application/json" \
  -d '{"published":true}'

# 9. Run full integration test
chmod +x test_full_flow.sh
./test_full_flow.sh
```

---

## 🎯 API Reference

### Authentication

| Endpoint | Method | Auth | Description |
|---|---|---|---|
| `/api/v1/auth/register` | POST | - | Register new user with email + password |
| `/api/v1/auth/login` | POST | - | Login, returns JWT token |
| `/api/v1/auth/me` | GET | JWT | Get current authenticated user |

**Request Examples:**

```bash
# Register
POST /api/v1/auth/register
{
  "email": "user@example.com",
  "password": "SecurePass123!"
}
# Response 201: {"id": "...", "email": "...", "created_at": "..."}

# Login
POST /api/v1/auth/login
{
  "email": "user@example.com",
  "password": "SecurePass123!"
}
# Response 200: {"token": "eyJ...", "user": {...}}
```

---

### Collections (Dynamic Content)

| Endpoint | Method | Auth | Description |
|---|---|---|---|
| `/api/v1/:collection` | POST | - | Create content in collection |
| `/api/v1/:collection` | GET | - | List contents from collection |
| `/api/v1/:collection/:id` | GET | - | Get single content by ID |
| `/api/v1/:collection/:id` | PUT | - | Full replace content data |
| `/api/v1/:collection/:id` | PATCH | - | Partial merge content data |
| `/api/v1/:collection/:id` | DELETE | - | Delete content |

**Collection Naming Rules:**
- Must start with lowercase letter
- Only `[a-z0-9_]` allowed
- 1-40 characters max
- Cannot start with `_` (reserved for system use)

**Valid:** `posts`, `products`, `blog_posts`, `v2_pages`  
**Invalid:** `Posts`, `my-posts`, `_internal`, `123posts`

**Request Examples:**

```bash
# Create in 'posts' collection
POST /api/v1/posts
{"title": "My Post", "content": "...", "published": true}
# Response 201: {"id": "...", "collection": "posts", "data": {...}, ...}

# List posts
GET /api/v1/posts
# Response 200: [{"id": "...", "collection": "posts", "data": {...}}, ...]

# Update (PUT = full replace)
PUT /api/v1/posts/<id>
{"title": "New Title", "content": "Updated content"}
# Old fields removed, only new fields remain

# Patch (PATCH = merge)
PATCH /api/v1/posts/<id>
{"published": false}
# Existing fields preserved, only 'published' updated
```

---

### Media (Uploads)

| Endpoint | Method | Auth | Description |
|---|---|---|---|
| `/api/v1/_media` | POST | JWT | Upload file via multipart form data |
| `/api/v1/_media` | GET | JWT | List current user's media |
| `/api/v1/_media/:id` | GET | JWT | Get media metadata |
| `/api/v1/_media/:id/download` | GET | JWT | Download file |
| `/api/v1/_media/:id` | DELETE | JWT | Delete media |

---

## 📁 Project Structure

```
rakit/
├── src/
│   ├── main.rs              # Entry point
│   ├── config/              # Env-based configuration
│   ├── db/                  # Pool + migrations
│   ├── api/
│   │   ├── mod.rs           # Router & AppState
│   │   ├── routes/          # Route grouping
│   │   └── handlers/        # HTTP handlers (thin)
│   ├── services/            # Business logic
│   ├── models/              # DB models / DTOs
│   ├── middleware/          # Auth middleware
│   ├── errors/              # Unified ApiError → HTTP
│   └── utils/               # Shared helpers (validation, etc.)
├── migrations/              # SQLx database migrations
├── tests/integration/       # End-to-end tests
├── docs/
│   ├── ROADMAP.md           # Feature roadmap
│   ├── ARCHITECTURE.md      # System design
│   └── plans/               # Per-feature implementation plans
├── docker-compose.yml       # Local Postgres
├── Dockerfile               # Production image
└── test_full_flow.sh        # Integration test script
```

---

## 🛣️ Roadmap

See [docs/ROADMAP.md](docs/ROADMAP.md) for detailed planning.

**Completed (v0.2):**
- ✅ Auth: JWT + Argon2 password hashing
- ✅ Full CRUD: Create, Read, Update (PUT/PATCH), Delete
- ✅ Collections: Content type namespacing
- ✅ Middleware: JWT authentication
- ✅ Validation: Collection names, passwords, emails

**Planned (v0.3+):**
- [ ] Query & Filter API (jsonb querying)
- [ ] RBAC: Role-based access control
- [ ] Schema Validation per collection
- [ ] Webhooks for content mutations
- [ ] Media/File uploads (S3-compatible)
- [ ] Admin dashboard (separate repo)
- [ ] GraphQL endpoint
- [ ] Real-time subscriptions

---

## 🧪 Testing

```bash
# Run full integration test
./test_full_flow.sh

# Manual curl tests
# See examples in "Try it" section above

# Unit tests (coming soon)
cargo test
```

---

## 🐳 Docker Deployment

```bash
# Build image
docker build -t rakit:latest .

# Run with docker-compose (includes Postgres)
docker compose up -d

# Or run standalone (bring your own DB)
docker run -p 3000:3000 \
  -e DATABASE_URL=postgres://user:pass@host/db \
  -e JWT_SECRET=your-secret \
  rakit:latest
```

---

## 🤝 Contributing

1. Fork the repository
2. Create feature branch from `main`
3. Follow existing code style (run `cargo fmt`)
4. Ensure `cargo clippy` passes
5. Write tests if applicable
6. Open Pull Request

See [docs/plans/](docs/plans/) for planned features.

---

## 📜 License

MIT - see [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

Built with:
- [Axum](https://github.com/tokio-rs/axum) — Web framework
- [SQLx](https://github.com/launchbadge/sqlx) — Async SQL toolkit
- [Tokio](https://tokio.rs/) — Async runtime
- [Argon2](https://github.com/RustCrypto/password-hashes) — Password hashing
- [jsonwebtoken](https://github.com/Keats/jsonwebtoken) — JWT encoding/decoding

Inspired by: Strapi, Directus, Payload CMS, Sanity.
