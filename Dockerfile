# Builder Stage
FROM rust:bookworm as builder
ENV SQLX_OFFLINE=true

# Create a new Rust project
RUN USER=root cargo new --bin rust_flutter_application
WORKDIR /rust_flutter_application

# Copy and build dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release --locked
RUN rm src/*.rs

# Copy the source code and build the application
COPY . .
RUN cargo build --release --locked

# Production Stage
FROM debian:bookworm-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get -y install ca-certificates tzdata openssl libssl3 \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /rust_flutter_application/target/release/rust_flutter_application ${APP}/rust_flutter_application

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

ENTRYPOINT ["./rust_flutter_application"]
EXPOSE 8000
