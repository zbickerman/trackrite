### TrackRite

A calorie & macro tracking backend, built as a learning project in Rust (Axum + SQLx + Postgres). This repo currently covers user authentication; food logging, diary tracking, and a companion FastAPI insights service are planned next (see Roadmap).

Status

🚧 Early stage — auth is built and tested. Diary/food logging is not yet implemented.

Stack
- Rust — Axum web framework, async via Tokio
- SQLx — async, compile-time-checked SQL queries against Postgres
- Postgres — running via Docker for local dev
- Argon2 — password hashing
- jsonwebtoken — JWT issuance for authenticated sessions

Features
✅ Done
- POST /auth/register — create a user; password is hashed with Argon2 before storage
- POST /auth/login — verify credentials, issue a signed JWT (30-day expiry)
- Passwords never stored in plaintext or returned in API responses
- Duplicate email / missing field / wrong password all handled gracefully (no crashes, no leaked info about which check failed)

🔜 Not yet built
- JWT verification middleware — tokens are issued but not yet enforced on any route
- Food catalog (manual entry + barcode lookup via Open Food Facts)
- Daily diary logging and calorie/macro summaries
- Companion FastAPI service for TDEE calculations, "best/worst foods" insights, and meal planning
- Automated tests (current testing has been manual, via Invoke-RestMethod/curl)
