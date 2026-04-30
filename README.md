<div align="center">

# ⚙️ RAKIT

**A lightweight, high-performance headless CMS engine built for the modern web.**

`Fast` · `Minimalist` · `Reliable`

</div>

---

## Why RAKIT?

- 🚀 **Blazing Fast** — Powered by Rust (Axum + SQLx).
- 🪶 **Low Footprint** — Runs on **< 20 MB RAM**.
- 🧑‍💻 **Developer First** — Simple JSON API, no bloat.
- 🧩 **Dynamic by Default** — Stores any content shape in PostgreSQL `jsonb`.

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

# 3. Run
cargo run
```

The server boots on `http://localhost:3000`.

### Try it

```bash
# Create a piece of content (any JSON shape)
curl -X POST http://localhost:3000/api/v1/contents \
  -H "Content-Type: application/json" \
  -d '{"title":"Hello RAKIT","tags":["intro","rust"],"published":true}'

# List
curl http://localhost:3000/api/v1/contents
```

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
│   ├── middleware/          # Auth, rate-limit, etc.
│   ├── errors/              # Unified ApiError → HTTP
│   └── utils/               # Shared helpers
├── migrations/              # SQLx migrations
├── tests/integration/       # End-to-end tests
├── docs/                    # Design notes
├── docker-compose.yml       # Local Postgres
└── Dockerfile               # Production image
```

---

## 🛣️ Roadmap

- [x] Core: dynamic `jsonb` content endpoint
- [ ] Auth: JWT + Argon2 password hashing
- [ ] Content types & schema validation
- [ ] Role-based access control
- [ ] Webhooks
- [ ] Admin UI (separate repo)

---

## 📜 License

MIT
