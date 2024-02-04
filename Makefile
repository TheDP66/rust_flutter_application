dev:
	docker-compose up -d

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
	cargo add actix-web
	cargo add actix-cors
	cargo add serde --features derive
	cargo add serde_json
	cargo add chrono --features serde
	cargo add env_logger
	cargo add dotenv
	cargo add argon2
	cargo add jsonwebtoken
	cargo add validator -F derive
	cargo add uuid --features "serde v4"
	cargo add sqlx --features "runtime-async-std-native-tls mysql chrono uuid"
	cargo install cargo-watch
	cargo install sqlx-cli
	cargo add utoipa -F "chrono actix_extras"
	cargo add utoipa-swagger-ui -F actix-web