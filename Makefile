.PHONY: dev migrate test fmt down

dev:
	docker compose up -d
	@echo "Waiting for PostgreSQL to be healthy..."
	@until docker compose exec -T postgres pg_isready -U postgres > /dev/null 2>&1; do \
  		echo "Postgres is not ready yet..."; \
  		sleep 2; \
	done
	@echo "PostgreSQL is ready ✅"
	cd backend && cargo run

migrate:
	cd backend && sqlx migrate run --source migrations

test:
	cd backend && cargo test
	cd web && pnpm test
	cd mobile && flutter test

fmt:
	cd backend && cargo fmt
	cd web && pnpm lint --fix
	cd mobile && dart format lib/

down:
	docker compose down -v
