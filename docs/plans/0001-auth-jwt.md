# Plan 0001 — Authentication (JWT + Argon2)

| Meta | Value |
|---|---|
| **Status** | ✅ Done |
| **Owner** | @you |
| **Depends on** | — |
| **Blocks** | 0005 RBAC, 0008 Media |
| **Effort** | M |
| **Created** | 2026-04-30 |

---

## 1. 🎯 Goal

Provide secure user authentication with:
- **Registration** — email + password, hashed with Argon2id
- **Login** — returns a signed JWT token for stateless sessions
- **Auth middleware** — protect endpoints by extracting & validating JWT
- **Current user endpoint** — `GET /auth/me` to fetch authenticated user info

## 2. 🚫 Non-goals

- Password reset flow (future)
- Email verification (future)
- OAuth / social login (future)
- Refresh tokens (keep stateless for now)
- Rate limiting (separate middleware, future)
- Role-based permissions (plan 0005)

## 3. 🌐 API Contract

### `POST /api/v1/auth/register`

**Request:**
```json
{
  "email": "user@example.com",
  "password": "SecurePass123!"
}
```

**Response 201:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "user@example.com",
  "created_at": "2026-04-30T12:00:00Z"
}
```

**Errors:**
- `400` — invalid email format or weak password
- `409` — email already exists

---

### `POST /api/v1/auth/login`

**Request:**
```json
{
  "email": "user@example.com",
  "password": "SecurePass123!"
}
```

**Response 200:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "created_at": "2026-04-30T12:00:00Z"
  }
}
```

**Errors:**
- `401` — invalid credentials

---

### `GET /api/v1/auth/me`

**Headers:**
```
Authorization: Bearer <token>
```

**Response 200:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "user@example.com",
  "created_at": "2026-04-30T12:00:00Z"
}
```

**Errors:**
- `401` — missing or invalid token

## 4. 🗄️ Data Model

### New table: `users`

```sql
CREATE TABLE IF NOT EXISTS users (
    id            UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email         TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_users_email_lower ON users (LOWER(email));
```

**Notes:**
- Email stored as-is, but indexed case-insensitively
- `password_hash` stores Argon2id output (~100 chars)
- `updated_at` trigger reused from existing pattern

## 5. 📁 File Changes

| File | Change | Purpose |
|---|---|---|
| `migrations/20260430000001_users.sql` | NEW | Users table + index |
| `src/models/user.rs` | NEW | User struct (no password_hash exposed) |
| `src/models/mod.rs` | UPDATE | Export user module |
| `src/services/auth.rs` | NEW | register, login, verify_token logic |
| `src/services/mod.rs` | UPDATE | Export auth module |
| `src/api/handlers/auth.rs` | NEW | HTTP handlers for auth endpoints |
| `src/api/handlers/mod.rs` | UPDATE | Export auth module |
| `src/api/routes/mod.rs` | UPDATE | Mount `/auth` routes |
| `src/middleware/auth.rs` | NEW | JWT extraction & validation middleware |
| `src/middleware/mod.rs` | UPDATE | Export auth middleware |
| `src/errors/mod.rs` | UPDATE | Add `Conflict` variant |

## 6. 🛠️ Implementation Steps

1. ✅ Write migration `20260430000001_users.sql`
2. ✅ Create `models/user.rs` with `User` struct (without password)
3. ✅ Create `services/auth.rs`:
   - `register(pool, email, password)` → hash with Argon2, insert, return User
   - `login(pool, email, password)` → verify hash, generate JWT, return (token, User)
   - `verify_token(token, secret)` → decode JWT, return Claims
4. ✅ Create `handlers/auth.rs`:
   - `register_handler` — validate input, call service, 201 response
   - `login_handler` — call service, 200 with token
   - `me_handler` — extract user from middleware extension, 200
5. ✅ Create `middleware/auth.rs`:
   - Axum middleware that extracts `Authorization: Bearer <token>`
   - Validates JWT, inserts user_id into request extensions
6. ✅ Wire routes in `api/routes/mod.rs`
7. ✅ Add `ApiError::Conflict` for duplicate email
8. ✅ Run migration, build, test manually

## 7. 🧪 Tests

### Manual verification with curl

```bash
# 1. Register
curl -X POST http://localhost:3000/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@rakit.dev","password":"SecurePass123!"}'

# Expected: 201 + user object

# 2. Login
curl -X POST http://localhost:3000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@rakit.dev","password":"SecurePass123!"}'

# Expected: 200 + {token, user}
# Copy the token

# 3. Get current user
curl http://localhost:3000/api/v1/auth/me \
  -H "Authorization: Bearer <paste-token-here>"

# Expected: 200 + user object

# 4. Verify duplicate email fails
curl -X POST http://localhost:3000/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@rakit.dev","password":"AnotherPass!"}'

# Expected: 409 Conflict
```

## 8. 🔒 Security Considerations

- ✅ **Argon2id** with default secure params (memory_cost=19456, time_cost=2, parallelism=1)
- ✅ **Case-insensitive email** via index to prevent duplicate accounts with different casing
- ✅ **JWT secret** from `JWT_SECRET` env var (warn if default)
- ✅ **Password validation** — min 8 chars (expand later with complexity rules)
- ✅ **Timing-safe comparison** — Argon2's `verify_password` is constant-time
- ⚠️ **Rate limiting** — deferred to future (add nginx/Caddy rate limit in production)
- ⚠️ **Token revocation** — not implemented (stateless = can't revoke until expiry)

## 9. ⚡ Performance Considerations

- **Argon2 hashing is CPU-intensive** — consider wrapping in `tokio::task::spawn_blocking` if /register becomes bottleneck (unlikely at current scale)
- **JWT validation** — fast, no DB hit needed (stateless)
- **Email index** — ensures fast duplicate checks on registration

## 10. ❓ Open Questions

- [x] **Refresh token strategy?** → Not implemented in v1; use long JWT expiry (24h) for now. Add refresh tokens in future plan.
- [x] **Where to store JWT on client side?** → Document in README: `localStorage` for web (accept XSS risk), `httpOnly` cookie better but needs CORS setup. Let devs choose.
- [x] **Token revocation list?** → Not implemented (stateless JWT = can't revoke). Workaround: short expiry + refresh tokens in future, or maintain blocklist table (plan 0009).

## 11. 📚 References

- [Argon2 RFC 9106](https://www.rfc-editor.org/rfc/rfc9106)
- [JWT best practices RFC 8725](https://www.rfc-editor.org/rfc/rfc8725)
