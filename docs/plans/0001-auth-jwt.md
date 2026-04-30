# Plan 0001 — Authentication (JWT + Argon2)

| Meta | Value |
|---|---|
| **Status** | 🟡 Draft (skeleton — to be filled by Opus-thinking) |
| **Owner** | @you |
| **Depends on** | — |
| **Blocks** | 0005 RBAC, 0008 Media |
| **Effort** | M |
| **Created** | 2026-04-30 |

---

## 1. 🎯 Goal

_TBD — Add user registration & login with secure password hashing (Argon2)
and stateless session via JWT. Provide a middleware that protects endpoints._

## 2. 🚫 Non-goals

_TBD — e.g. password reset flow, email verification, OAuth — defer to later plans._

## 3. 🌐 API Contract

_TBD — `POST /auth/register`, `POST /auth/login`, `GET /auth/me`._

## 4. 🗄️ Data Model

_TBD — `users` table: id, email (unique), password_hash, role, created_at, updated_at._

## 5. 📁 File Changes

_TBD — to be enumerated during planning._

## 6. 🛠️ Implementation Steps

_TBD._

## 7. 🧪 Tests

_TBD._

## 8. 🔒 Security Considerations

_TBD — Argon2 params, JWT secret rotation, timing attacks, rate limit on /login._

## 9. ⚡ Performance Considerations

_TBD._

## 10. ❓ Open Questions

- [ ] Refresh token strategy? (rotating vs long-lived)
- [ ] Where to store JWT on client side? (advice in docs)
- [ ] Token revocation list?

## 11. 📚 References

- [Argon2 RFC 9106](https://www.rfc-editor.org/rfc/rfc9106)
- [JWT best practices RFC 8725](https://www.rfc-editor.org/rfc/rfc8725)
