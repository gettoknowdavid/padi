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

1. Copy `.env.example` to `.env` and fill in values
2. `make dev`         — starts Docker services + backend
3. `make migrate`     — runs database migrations
4. `make test`        — runs all tests
5. `make fmt`         — formats all code