# Padi CRM

> A CRM platform built for Nigerian businesses.

## Stack
- **Backend**: Rust / Axum
- **Web**: Next.js 15 / TypeScript / TailwindCSS
- **Mobile**: Flutter 3.x

## Prerequisites
- Rust (stable) — install via [rustup](https://rustup.rs)
- Node.js 20+ + pnpm
- Flutter 3.x
- Docker + Docker Compose

## Getting Started

```bash
# 1. Clone the repo
git clone <repo-url> && cd padi

# 2. Copy env file and fill in values
cp .env.example .env

# 3. Start local services (PostgreSQL + Redis)
make dev          # added in E1-T6

# 4. Run backend
cd backend && cargo run

# 5. Run web
cd web && pnpm dev

# 6. Run mobile
cd mobile && flutter run
```