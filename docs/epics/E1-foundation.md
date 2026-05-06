# E1 — Foundation & Infrastructure
**Goal**: Set up the monorepo, backend skeleton, database, CI/CD, and local development environment so every subsequent ticket has a working foundation to build on.

**Stack**: Rust/Axum, PostgreSQL (Supabase), Redis (Upstash), Docker, GitHub Actions

---

## Tickets

| ID | Title | Status |
|----|-------|--------|
| E1-T1 | Monorepo scaffold + toolchain setup | ⬜ |
| E1-T2 | Backend: Axum skeleton + config + health check | ⬜ |
| E1-T3 | Database: PostgreSQL connection + base migrations | ⬜ |
| E1-T4 | Redis connection + rate limiting middleware | ⬜ |
| E1-T5 | GitHub Actions CI pipeline | ⬜ |
| E1-T6 | Docker Compose local dev environment | ⬜ |

---

## E1-T1 — Monorepo Scaffold + Toolchain Setup

**Goal**: Initialize the monorepo with the correct folder structure, Rust workspace, and toolchain pinning.

**Acceptance Criteria**:
- [ ] Root `padi/` directory with `backend/`, `web/`, `mobile/`, `docs/` as per TRD folder structure
- [ ] `backend/` is a valid Rust workspace with a single `padi-api` crate
- [ ] `.cargo/rust-toolchain.toml` pinned to stable (e.g., `1.77.0`)
- [ ] `web/` initialized as a Next.js 15 project with TypeScript + Tailwind + shadcn/ui
- [ ] `mobile/` initialized as a Flutter 3.x project
- [ ] `.gitignore` covers Rust (`target/`), Node (`node_modules/`), Flutter (`.dart_tool/`, `build/`), and env files
- [ ] `README.md` at root with setup instructions

**Implementation Notes**:
- Use `cargo new --lib` for the backend crate
- Run `npx create-next-app@latest web --typescript --tailwind --app` for web
- Run `flutter create mobile` for mobile
- Add `.env.example` with all required env var keys (no values)

**Required Env Vars** (document in `.env.example`):
```
DATABASE_URL=
REDIS_URL=
JWT_SECRET=
JWT_REFRESH_SECRET=
PAYSTACK_SECRET_KEY=
FLUTTERWAVE_SECRET_KEY=
WHATSAPP_TOKEN=
WHATSAPP_PHONE_NUMBER_ID=
AFRICAS_TALKING_API_KEY=
AFRICAS_TALKING_USERNAME=
RESEND_API_KEY=
CLOUDFLARE_R2_ACCOUNT_ID=
CLOUDFLARE_R2_ACCESS_KEY=
CLOUDFLARE_R2_SECRET_KEY=
CLOUDFLARE_R2_BUCKET=
APP_ENV=development
PORT=8080
FRONTEND_URL=http://localhost:3000
```

**Files to create**:
```
padi/
├── Cargo.toml (workspace)
├── .gitignore
├── .env.example
├── README.md
├── backend/
│   ├── Cargo.toml
│   └── src/main.rs (placeholder)
├── web/  (next.js init)
└── mobile/ (flutter init)
```

---

## E1-T2 — Backend: Axum Skeleton + Config + Health Check

**Goal**: Build the Axum application skeleton with proper config loading, structured logging, and a `/health` endpoint.

**Acceptance Criteria**:
- [ ] Server starts and listens on `PORT` from env
- [ ] `GET /health` returns `{ "status": "ok", "version": "0.1.0" }` with 200
- [ ] Config struct loads all env vars at startup; fails fast with clear error if required vars are missing
- [ ] Structured JSON logging via `tracing` + `tracing-subscriber` (log level from `RUST_LOG` env)
- [ ] Graceful shutdown on SIGTERM/SIGINT
- [ ] Error handling: custom `AppError` enum that implements `IntoResponse`
- [ ] Response envelope: all API responses wrapped in `{ "data": ..., "meta": ..., "error": null }`

**Dependencies** (add to `Cargo.toml`):
```toml
axum = { version = "0.7", features = ["macros"] }
tokio = { version = "1", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace", "timeout"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
dotenvy = "0.15"
thiserror = "1"
anyhow = "1"
```

**File structure**:
```
backend/src/
├── main.rs          # Tokio runtime, graceful shutdown
├── app.rs           # Router assembly, middleware stack
├── config.rs        # Config struct + from_env()
├── errors.rs        # AppError + IntoResponse impl
└── routes/
    └── health.rs    # GET /health handler
```

**Test**: `cargo test` passes. `curl localhost:8080/health` returns expected JSON.

---

## E1-T3 — Database: PostgreSQL Connection + Base Migrations

**Goal**: Connect to PostgreSQL via SQLx with a connection pool, set up the migrations system, and create the base schema migrations.

**Acceptance Criteria**:
- [ ] `PgPool` initialized at startup, stored in Axum state
- [ ] `sqlx migrate run` executes all migrations successfully
- [ ] Base migrations create all tables defined in TRD Section 8 (data model)
- [ ] Indexes on: `contacts.primary_phone`, `contacts.org_id`, `deals.org_id`, `deals.stage_id`, `communications.contact_id`, `audit_logs.org_id`
- [ ] Migration tests: `sqlx migrate run` is idempotent (run twice = no error)
- [ ] `.sqlx/` query cache committed to repo for offline `cargo build`

**Dependencies**:
```toml
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres", "uuid", "chrono", "json", "migrate"] }
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
```

**Migration files** (`backend/migrations/`):
```
0001_create_organizations.sql
0002_create_users.sql
0003_create_organization_members.sql
0004_create_contacts.sql
0005_create_companies.sql
0006_create_contact_companies.sql
0007_create_pipelines_and_stages.sql
0008_create_deals.sql
0009_create_tasks.sql
0010_create_communications.sql
0011_create_message_templates.sql
0012_create_invoices.sql
0013_create_payments.sql
0014_create_audit_logs.sql
0015_create_settings.sql
0016_create_indexes.sql
```

**Important**: All monetary values stored as `BIGINT` (kobo). All IDs as `UUID`. All timestamps as `TIMESTAMPTZ`.

---

## E1-T4 — Redis Connection + Rate Limiting Middleware

**Goal**: Connect to Redis (Upstash) and implement sliding window rate limiting middleware for all API routes.

**Acceptance Criteria**:
- [ ] Redis client initialized at startup (use `deadpool-redis` for pooling)
- [ ] Rate limiting middleware: 100 requests per 10 seconds per user (by JWT user ID if authenticated, by IP if not)
- [ ] Returns `429 Too Many Requests` with `Retry-After` header when limit exceeded
- [ ] Auth endpoints have stricter limit: 5 per 15 minutes per IP
- [ ] Rate limit middleware is registered in the Axum middleware stack
- [ ] Unit test: assert 429 is returned after N+1 requests in window

**Dependencies**:
```toml
deadpool-redis = "0.14"
```

**Implementation**: Use Redis `ZADD`/`ZREMRANGEBYSCORE`/`ZCARD` sliding window algorithm. Key format: `rate_limit:{user_id_or_ip}:{endpoint_group}`.

---

## E1-T5 — GitHub Actions CI Pipeline

**Goal**: Set up CI to run on every push and PR: lint, test, build check for all three apps.

**Acceptance Criteria**:
- [ ] CI runs on push to `main` and all PRs
- [ ] Backend job: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`
- [ ] Web job: `pnpm lint`, `pnpm type-check`, `pnpm test` (Vitest)
- [ ] Mobile job: `flutter analyze`, `flutter test`
- [ ] CI uses caching for Rust `target/`, `~/.cargo/registry`, `node_modules`, Flutter pub cache
- [ ] CI fails fast if any job fails
- [ ] `DATABASE_URL` for tests uses a PostgreSQL service container in Actions

**File**: `.github/workflows/ci.yml`

**Notes**: Use `rust-cache` action for Rust. Use `setup-flutter` action. Keep CI under 10 minutes total by caching aggressively.

---

## E1-T6 — Docker Compose Local Dev Environment

**Goal**: Single `docker-compose up` command starts the full local dev environment (PostgreSQL, Redis, and optionally the backend).

**Acceptance Criteria**:
- [ ] `docker-compose up -d` starts PostgreSQL 16 + Redis 7
- [ ] PostgreSQL exposed on `5432`, Redis on `6379`
- [ ] PostgreSQL initialized with a `padi_dev` database and `padi` user
- [ ] Health checks on both services
- [ ] `docker-compose down -v` cleanly removes volumes
- [ ] `Makefile` with shortcuts: `make dev` (start docker + run backend), `make migrate` (run migrations), `make test` (run all tests), `make fmt` (format all)
- [ ] README updated with "Getting Started" using `make dev`

**Files**:
- `docker-compose.yml`
- `Makefile`
- `.docker/postgres/init.sql` (create db + user)
