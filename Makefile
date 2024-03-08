dev:
	docker-compose -f docker-compose.no_api.yml up -d

dev-api:
	make dev
	cargo sqlx prepare
	docker-compose -f docker-compose.yml up -d

dev-down:
	docker-compose down

new-migrate:
	sqlx migrate add -r $(name)

migrate-up:
	sqlx migrate run

migrate-down:
	sqlx migrate revert

server:
	cargo watch -q -c -w src/ -x run

server-up:
	ngrok http 8090

install:
	cargo install cargo-watch sqlx-cli